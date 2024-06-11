#!/bin/bash

for i in {1..100}; do
  curl -X 'POST' \
  'http://localhost:3000/api/v1/questions/add' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d "{
  \"content\": \"Content$i\",
  \"id\": $i,
  \"tags\": [
    \"history\",
    \"math\"
  ],
  \"title\": \"Title$i\"
}"
done
