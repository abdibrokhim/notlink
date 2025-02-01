some API testing sutff:

example post:
https://www.linkedin.com/posts/abdibrokhim_i-know-you-are-struggling-to-find-an-idea-activity-7285736275242864644-ehpY?utm_source=share&utm_medium=member_desktop

[shorten]

[shorten_url]:
for testing the shorten endpoint, you can use the following curl command:
curl -X POST -H "Content-Type: application/json" \
     -d '{"original_url":"https://www.linkedin.com/posts/abdibrokhim_i-know-you-are-struggling-to-find-an-idea-activity-7285736275242864644-ehpY?utm_source=share&utm_medium=member_desktop"}' \
     http://localhost:8000/shorten

then open browser and go to http://localhost:8000/replace_with_your_short_code to see the redirect in action.

[example_response]:
{"id":1,"original_url":"https://www.linkedin.com/posts/abdibrokhim_i-know-you-are-struggling-to-find-an-idea-activity-7285736275242864644-ehpY?utm_source=share&utm_medium=member_desktop","short_code":"TTMCeh","created_at":"2025-01-16T21:13:55.281419"}


[wallet]

[create_wallet]:
for testing the wallet creation endpoint, you can use the following curl command:
curl -X POST -H "Content-Type: application/json" \
     -d '{"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL"}' \
     http://localhost:8000/wcreate

[example_response]:
{"id":1,"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL","tries_left":5,"created_at":"2025-01-17T00:53:44.988324"}

my sol address: 6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL . feel free to send some sol to me. LOL

[update_wallet_tries]:
curl -X POST -H "Content-Type: application/json" \
     -d '{"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL"}' \
     http://localhost:8000/wupdate

[example_response]:
{"id":1,"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL","tries_left":4,"created_at":"2025-01-17T00:53:44.988324"}


[encrypt]

[encrypt_url]:
curl -X POST -H "Content-Type: application/json" \
     -d '{"original_url":"https://x.com/search?q=%24TRUMP&src=trend_click&vertical=trends", "encrypt": true}' \
     http://localhost:8000/shorten


[expire]

[expire_url]:
curl -X POST -H "Content-Type: application/json" \
     -d '{"original_url":"https://www.linkedin.com/posts/abdibrokhim_im-building-blazingly-fast-url-shortener-activity-7286433490663862274-zvKb?utm_source=share&utm_medium=member_desktop", "encrypt": true, "transaction_hash": "34xPKazmdeJyJtpTyiQSTo848YEs1AetzbSJnQLUmhPBWK4nB4xRJY5fpA244YQrjh5xNDAHUrmYVEP6oWKoqSWK"}' \
     http://localhost:8000/shorten

[example_response]:
{"id":3,"original_url":"eNp0KTozH9yadjsnW/FtspaNbPX48LOiV6HTc+LhUb7tr+AGHCdGFPBhZAkipIUaxv6HJUzl++f9CvJtQ/vbtHW+Zq9tPv1NpmJ0sYJB2uWX28UYOkq10JOjEHeujCOYyCIdgqO+dNZCLWB710DYMOC6ioof1XdTBpbJAhdDpvihqmvAPcRVgnmi1fiQiA5nGfXz5azHEf1XB02bVsmXxWV2x1nTgMTKh0hf2S8sbVx8hIFO4FH3e4eEg4g=","short_code":"fYG4cp","created_at":"2025-01-18T20:45:39.412587","encrypted":true,"expired":false,"transaction_hash":"34xPKazmdeJyJtpTyiQSTo848YEs1AetzbSJnQLUmhPBWK4nB4xRJY5fpA244YQrjh5xNDAHUrmYVEP6oWKoqSWK"}


[expire_url_immediate]:
curl -X POST -H "Content-Type: application/json" \
     -d '{"short_url_id": 3}' \
     http://localhost:8000/expire

ps: it works only for paid urls. i mean there should be transaction_hash in the request body.


[get_all_short_urls]

curl -X GET http://localhost:8000/all-urls

