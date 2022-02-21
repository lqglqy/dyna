#!/usr/bin/python3
import json
import base64
from pysqlcipher3 import dbapi2 as sqlite

def getKeywordById(kid):
    conn = sqlite.connect('signature_v002_2021-12-28.db')
    c = conn.cursor()
    c.execute("PRAGMA key='JBz1OlZ0n5UiCQvD'")
    sql = "select border_check, key_word, key_word_id from key_word where key_word_id = {:d}".format(kid)
    c.execute(sql)
    conn.commit()
    keyword = {}
    for row in c:
        keyword['id'] = row[2]
        keyword['key_word'] = row[1]
        keyword['border_check'] = row[0]
    c.close()
    return keyword
 


def getChainRuleById(sid):
    conn = sqlite.connect('signature_v002_2021-12-28.db')
    c = conn.cursor()
    c.execute("PRAGMA key='JBz1OlZ0n5UiCQvD'")
    c.execute('''select regexp,meet_target,operator from signature_rule where signature_id=\"'''+sid+"\"")
    conn.commit()
    for row in c:
        regx = row[0]
        meet_target = row[1]
        oper = row[2]
    c.close()

    rule = meet_target + " matches \"" + base64.b64encode(regx.encode()).decode() + "\""
    if oper == "nrx":
        rule = "not " + rule
    return "( " + rule + ")"
def getKeywordAndById(kid):
    conn = sqlite.connect('signature_v002_2021-12-28.db')
    c = conn.cursor()
    c.execute("PRAGMA key='JBz1OlZ0n5UiCQvD'")
    sql = "select border_check, key_word, key_word_id from key_word_and where key_word_id = {:d}".format(kid)
    c.execute(sql)
    conn.commit()
    keyword = {}
    for row in c:
        keyword['id'] = row[2]
        keyword['key_word'] = row[1]
        keyword['border_check'] = row[0]
    c.close()
    return keyword
 
def getKeyword(signature_id, meet_target):
    conn = sqlite.connect('signature_v002_2021-12-28.db')
    c = conn.cursor()
    c.execute("PRAGMA key='JBz1OlZ0n5UiCQvD'")
    sql = '''select key_word_id from key_word_map where signature_id = \"'''+ signature_id + "\""
    c.execute(sql)
    conn.commit()
    keyword = ""
    keymap = ['none', 'left', 'right', 'both']
    mtarget = meet_target

    # use header match keyword
    if mtarget == "RESPONSE_STATUS":
        mtarget = "RESPONSE_HEADER"
    for row in c:
        res = getKeywordById(row[0])
        if len(res) != 0:
            cur = "prefilter(keyword, \"{}\",\"{:d}\",{},\"{}\")".format(mtarget, res['id'], json.dumps(res['key_word']), keymap[res['border_check']])
            if keyword != "":
                keyword = keyword + " || " + cur
            else:
                keyword = cur
        else:
            res = getKeywordAndById(row[0])
            cur = "prefilter(keyword, \"{}\",\"{:d}\",{},\"{}\")".format(mtarget, res['id'], json.dumps(res['key_word']), keymap[res['border_check']])
            if keyword != "":
                keyword = keyword + " && " + cur
            else:
                keyword = cur   

    c.close()
    return keyword
    
def generatorRule( chain_flag, signature_id,signature_chain_index,regexp,support_key_word,meet_target,operator ):
    rule = {}
    rule['id'] = signature_id
    if support_key_word == 1:
        keyword = getKeyword(signature_id, meet_target)
        rule['rule'] = keyword
        #rule['rule'] = rule['rule'] + " && (" + meet_target + " matches \"" + regexp + "\")"
        #rule['rule'] = rule['rule'] + " && (" + meet_target + " matches " + json.dumps(regexp) + ")"
        rule['rule'] = rule['rule'] + " && (" + meet_target + " matches \"" + base64.b64encode(regexp.encode()).decode() + "\")"
        if signature_chain_index != '000000000':
            chain_rule = getChainRuleById(signature_chain_index)
            rule['rule'] = rule['rule'] + " && " + chain_rule
    elif chain_flag == 0:
            #rule['rule'] = "(" + meet_target + " matches \"" + regexp + "\")"
            #rule['rule'] = "(" + meet_target + " matches " + json.dumps(regexp) + ")"
            rule['rule'] = "(" + meet_target + " matches \"" + base64.b64encode(regexp.encode()).decode() + "\")"
    else:
        return None
    return rule
def main():    
    conn = sqlite.connect('signature_v002_2021-12-28.db')
    c = conn.cursor()
    c.execute("PRAGMA key='JBz1OlZ0n5UiCQvD'")
    c.execute('''select chain_flag,signature_id,signature_chain_index,netlogic_regexp,support_key_word,meet_target,operator,regexp from signature_rule where direction=2''')
    conn.commit()
    i = 0
    for row in c:
        #print(row)
        #regex = row[3]
        #if regex is None:
        regex = row[7]
        rule = generatorRule(row[0], row[1], row[2], regex, row[4], row[5], row[6])
        i=i+1
        #print(i)
        if rule is not None:
            print(json.dumps(rule))
    c.close()

if __name__ == '__main__':
    main()
            
        
    