# [tothefuture.me](https://www.tothefuture.me/)

Only blazingly fast 🚀🚀🚀🚀🚀🚀🚀

Welcome to the tothefuture.me! This is a website for sending e-mails into future. (used to be named timecapsule-rs but I didn't like the name)

Basically functions like a digital timecapsule. Instead of using physical capsules, you use e-mails! Write an e-mail, set it's date and the e-mail will arrive at you at that date, to the e-mail adress you have signed up with.

You can view mails you have created, edit them, delete them, preview them by getting the e-mail sent to you. It even has HTML editor for you to take maximum control. Once you are done, you can hide the mail so that you won't get spoiled until the mail has been sent to you.

If the premise sounded good enough for you, why don't you try it yourself. Write a short e-mail and forget about it until you see it in your mail box!

[www.tothefuture.me ](https://www.tothefuture.me/)

## This website is still pretty much work in progress.

Technically, this website uses rust in backend and sveltekit in frontend. I'm using postgresql for persistent data, redis for short term data. They are all planned to be hosted in railway.

I have plans to use rabbitMQ for e-mail sending routine. And also an image server for users to upload images and later add them to e-mails.

Big Milestones:
- [x] Text editor
- [x] Basic funtionalities
- [ ] Mail sending service
- [ ] Mail account activation
- [ ] Image server

You can track [issues](https://github.com/112batuhan/timecapsule-rs/issues) for planned additions and changes as I keep my smaller tasks there.
