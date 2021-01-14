import requests
import json
s = requests.Session()
h = "http://127.0.0.1:5000"
req = {"email":"t1@mail.com", "password":"testpass"}
resp = s.post(h+"/register", json=req)
resp2 = s.get(h+f"/register/confirm?code={resp.content}")
resp2.content
resp2
s.get(h+"/dev/whoami")
s.get(h+"/dev/whoami").content
req = {"email":"t1@mail.com", "password":"testpass", "remember":False}
resp3 = s.post(h+"/login", json=req)
resp4 = s.get(h+"/dev/whoami")
resp4
resp4.content