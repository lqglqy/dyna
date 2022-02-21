#!/usr/bin/python3
import json
import base64
from pysqlcipher3 import dbapi2 as sqlite

def main():    
    conn = sqlite.connect('signature_v002_2021-12-28.db')
    c = conn.cursor()
    c.execute("PRAGMA key='JBz1OlZ0n5UiCQvD'")
    c.execute('''select signature_id,attack_example from signature_rule where direction=2 and chain_flag=0''')
    conn.commit()
    for row in c:
        #print(row)
        attack = row[1].replace("<#", "")
        attack = attack.replace("#>", "")
        
        fo = open("test_case/"+row[0], "w")
        fo.write(attack)
        fo.close()
    c.close()

if __name__ == '__main__':
    main()
            
        
    