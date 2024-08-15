# Honeywell Total Comform interface

This project is intended to be a pure Rust inferface to interact with Honwywell Total Comfort APIs

## Project setup
Set the environmental variable `HONEYWELL_USERNAME` and `HONEYWELL_PASSWORD` according to your personal credential.

Set `HONEYWELL_APPLICATION_ID=91db1612-73fd-4500-91b2-e63b069b185c`

### Test API

#### Authenticate
```bash
curl -X POST https://mytotalconnectcomfort.com/WebApi/api/Session \
-H "accept: application\json" \
-H "Content-Type: application/json" \
-d '{
  "username": "'"${HONEYWELL_USERNAME}"'",
  "password": "'"${HONEYWELL_PASSWORD}"'",
  "applicationId": "'"${HONEYWELL_APPLICATION_ID}"'"
}'
```

#### Get Locations
```bash
curl -X GET https://mytotalconnectcomfort.com/WebApi/api/locations?userId=<user-id>&allData=true \
-H "sessionId": "<session-id>"
```

#### Get Devices
```bash
curl -X GET https://mytotalconnectcomfort.com/WebApi/api/devices?locationId=<location-id>&allData=true \
-H "sessionId": "<session-id>"
```

# Reference

* [Home Automation: Using Power Automate to Access the Honeywell Evohome API](https://elliskarim.com/2022/12/04/home-automation-using-power-automate-to-access-the-honeywell-evohome-api/)