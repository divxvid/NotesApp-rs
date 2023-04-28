import requests
import json
from decorate import decorate

URL = "http://localhost:8080"
sesh = requests.Session()

@decorate
def test_login(username, password):
    print(f"Testing Login Path for Username: {username} and Password: {password}")
    data = {"username": username, "password": password}
    r = sesh.post(URL + "/login", json=data)
    print(r)


@decorate
def test_signup(username, password):
    print(f"Testing Signup Path for Username: {username} and Password: {password}")
    creds = {"username": username, "password": password}
    r = sesh.post(URL + "/signup", json=creds)
    print(r)
    print(r.text)


@decorate
def test_root():
    print("Testing Root Path")
    r = sesh.get(URL + "/")
    print(r)
    print(r.text)


@decorate
def test_add_note(note):
    print(f"Testing Add Note for Note: {note}")
    r = sesh.post(URL + "/notes", json=note)
    print(r)
    print(r.text)


@decorate
def test_get_all_notes():
    print("Testing Get All Notes")
    r = sesh.get(URL + "/notes")
    print(r)
    if r.status_code == 403:
        return None
    print(r.json())
    return r.json()


@decorate
def test_get_note(id):
    print(f"Testing Get Single Note for ID: {id}")
    r = sesh.get(URL + "/notes/" + id)
    print(r)
    print(r.text)


@decorate
def test_delete_note(id):
    print(f"Testing Delete Single Note for ID: {id}")
    r = sesh.delete(URL + "/notes/" + id)
    print(r)
    print(r.text)

@decorate
def test_logout():
    print("Testing Logout!")
    r = sesh.get(URL + "/logout")
    print(r)
    print(r.cookies)
    # print(r.json())


if __name__ == "__main__":
    username = "TestUsername1"
    password = "TestPassword1"
    test_root()
    # test_signup(username, password)
    test_login(username, password)

    # note1 = {"title": "Title1", "note": "Test Note 1"}
    # note2 = {"title": "Title2", "note": "Test Note 2"}
    # test_add_note(note1)
    # test_add_note(note2)
    # notes = test_get_all_notes()
    # print(notes)
    # ids = []
    # for note in notes:
    #     test_get_note(note["_id"])
    #     ids.append(note["_id"])
    #
    # for id in ids:
    #     test_delete_note(id)
    #
    # test_get_all_notes()
    #
    test_logout()
    test_logout()
    # test_get_all_notes()


