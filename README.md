# timecapsule-rs

Only blazingly fast 🚀🚀🚀🚀🚀🚀🚀

Welcome to the Timecapsule! Timecapsule is a website for sending e-mails into future. Name is not final and it's just a code name at this stage. 

Basically functions like a physical timecapsule, but with e-mails. Write an e-mail, set it's date and the e-mail will arrive at you at that date, to the e-mail adress you have signed up with.

You can view mails you have created, edit them, delete them, preview them by getting the e-mail sent to you. It even has HTML editor for you to take maximum control. Once you are done, you can hide the mail so that you won't get spoiled until the mail has been sent to you. (some of the features I listed are still in development. Sorry for that.)

If the premise sounded good enough for you, why don't you try it yourself. Write a short e-mail and forget about it until you see it in your mail box!

https://timecapsule.up.railway.app/ (Domain will change after picking up a permanent name.)

## This website is still pretty much work in progress.

Technically, this website uses rust in backend and sveltekit in frontend. I'm using postgresql for persistent data, redis for short term data. They are all planned to be hosted in railway.

I have plans to use rabbitMQ for e-mail sending routine. And also an image server for users to upload images and later add them to e-mails.

Big Milestones:
- [x] Text editor
- [ ] Mail sending service
- [ ] Mail account activation
- [ ] Image server

You can track [issues](https://github.com/112batuhan/timecapsule-rs/issues) for planned additions and changes as I keep my smaller tasks there.
