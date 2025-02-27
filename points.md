Why Might You Still Want the /expire Endpoint?
Manual/Immediate Expiration:

The /expire endpoint allows you (or the user) to forcibly expire the link before 24 hours, e.g., “I just want to shut this link down now.”
It checks transaction_hash.is_some() so only paid links can be forcibly expired.

How it works:

user must connect wallet first to use the app.
then they have free 5 tries to shorten urls.
after that they have to pay $0.99 usd for each url shortening.
if they wanna use advanced features, they have to pay extra $0.99 usd for each url shortening.