built with rust, shuttle, actix web, neon, diesel.

rust: for programming language.
shuttle: to deploy our backend to the cloud.
actix web: for web framework.
neon: for database. saving the data, and etc. it's postgresql.
diesel: for ORM. we should use postgresql.

1. create a new rust project.

2. setup diesel with postgresql.

3. setup shuttle. also integrated actix web.

4. setup neon. setup postgresql.

5. we created sample api endpoint hello world.

6. we created tables, and did migration, and also did alteration.

7. we wrote functions; operations.

8. we wrote models for our tables.

9. mod.rs everytime we did reimport the modules.

10. we define the routes; for url shortening and redirection.

11. we wrote helper functions; crypto.rs for encryption and decryption. also, we wrote a functions to get the stuff from .env file or in our case Secrets.toml. and also random code generator with ... length. (combination of letters and numbers).

12. we updated the main.rs file to use the routes. to get connected with the database. and start the server.

13. we deployed our backend to the cloud. with shuttle. in case issues with libpq.so.5 we wrote custom bash script to install the libpq-dev. we did another deployment with the custom bash script. and it worked. now we have our backend running on the cloud.

14. we tested our backend with curl. and it worked.

15. we did other bunch of stuffs. please kindly check out the video tutorial for more details.

that's it!!

Watch the full video tutorial on YouTube [How I Built Open Source Fastest URL Shortener Ever With Rust](https://youtu.be/Mhz_eVH3XnQ). (put like and subscribe if you found it useful).

Github repo for this tutorial with all the details, secrets, env files available here [github.com/abdibrokhim/notlink-tutorial](https://github.com/abdibrokhim/notlink-tutorial). (dont forget to put a star ⭐️ on this official repo).

[![Watch the full video tutorial on YouTube](assets/cover.png)](https://youtu.be/Mhz_eVH3XnQ)

find me here: [yaps.gg](https://yaps.gg). (im not yapping, but i do yap sometimes).