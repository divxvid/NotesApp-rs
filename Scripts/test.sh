#!/bin/bash

echo -e "Testing POST /signup"
curl --verbose --header "Content-Type: application/json" \
	--request POST \
	--data '{"username":"Rust_is_OP", "password": "CrabLangLuL"}' \
	http://localhost:3000/signup
echo -e "\n-----------------------------------------------"

echo -e "\n\nTesting POST /login"
curl --verbose --header "Content-Type: application/json" \
	--request POST \
	--data '{"username":"test_username", "password": "pass123"}' \
	http://localhost:3000/login
echo -e "\n-----------------------------------------------"

echo -e "\n\nTesting GET /notes/:id"
curl --verbose http://localhost:3000/notes/123
echo -e j"\n-----------------------------------------------"

echo -e "\n\nTesting GET /notes"
curl --verbose http://localhost:3000/notes
echo -e "\n-----------------------------------------------"

echo -e "\n\nTesting POST /notes"
curl --verbose --header "Content-Type: application/json" \
	--request POST \
	--data '{"title":"test_title", "note": "note body"}' \
	http://localhost:3000/notes
echo -e "\n-----------------------------------------------"

echo -e "\n\nTesting DELETE /notes/:id"
curl --verbose --request DELETE http://localhost:3000/notes/123
echo -e "\n-----------------------------------------------"
