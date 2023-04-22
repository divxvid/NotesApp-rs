#!/bin/bash

echo -e "Testing POST /signup"
curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"username":"test_username", "password": "pass123"}' \
	http://localhost:3000/signup

echo -e "\n\nTesting POST /login"
curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"username":"test_username", "password": "pass123"}' \
	http://localhost:3000/login

echo -e "\n\nTesting GET /notes/:id"
curl http://localhost:3000/notes/123

echo -e "\n\nTesting GET /notes"
curl http://localhost:3000/notes

echo -e "\n\nTesting POST /notes"
curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"title":"test_title", "note": "note body"}' \
	http://localhost:3000/notes

echo -e "\n\nTesting DELETE /notes/:id"
curl --request DELETE http://localhost:3000/notes/123
