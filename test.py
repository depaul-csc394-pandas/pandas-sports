import dotenv
import os
import requests
import yaml

METHODS = ['post', 'get', 'delete']

dotenv.load_dotenv()

d = os.getenv("DOMAIN")
if d is not None:
    DOMAIN = d
else:
    DOMAIN = 'localhost'

p = os.getenv("PORT")
if p is not None:
    PORT = int(p)
else:
    PORT = 8080

URL = 'http://{}:{}/api'.format(DOMAIN, PORT)

def validate_request(req):
    if req['method'] is None or req['method'] not in METHODS:
        print('Request has invalid method or method is None:\n{}'.format(req))
        return False

def check_status(req, resp):
    if resp.status_code != req['response']['status']:
        print('Unexpected status code for request (expected {}, got {})'
              .format(req['response']['status'], resp.status_code))
        print('Request: {}'.format(req))
        print('Response: {}'.format(resp.text))
        exit(1)

def set_cookie(session, header_val):
    args = header_val.split('; ')
    session.headers['cookie'] = args[0]

with open('test-data.yaml') as yaml_file:
    test_data = yaml.load(yaml_file)

session = requests.Session()
# log in as admin
login_data = {
    "username": "admin",
    "password": os.getenv("ADMIN_PASS"),
}
print('login_data: {}'.format(login_data))
login_response = session.post(URL + '/login', json=login_data)
login_response.raise_for_status()
set_cookie(session, login_response.headers['set-cookie'])

reqs = test_data['requests']
for req in reqs:
    validate_request(req)
    method = req['method'].lower()
    full_url = URL + req['route']
    if method == 'post':
        resp = session.post(full_url, json=req['body'])
    elif method == 'get':
        resp = session.get(full_url)
    elif method == 'delete':
        resp = session.delete(full_url)
    else:
        print('Method ({}) not recognized!'.format(method))
        exit(1)
    check_status(req, resp)

