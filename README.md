# Intro

Rust [Actix web framework](https://github.com/actix/actix-web) serving static [solidjs](https://github.com/solidjs/) files built by [vite](https://github.com/vitejs/vite). By no means it is ready for anything, there is no SSR for starters. I only wanted to know I could not if I should.

If you really desire to serve JS using Rust I'd keep an eye on things like [create-rust-app](https://github.com/Wulf/create-rust-app)

For now, I call this "Rusty Solid" which incidentally would be my adult performer name.

## Why

I don't know. Wanted to try actix web I guess. I've also never written a line of Solidjs before .. which you can tell by the commit history.

Btw can we all appreciate how sick the js bundle size is, I'm sure that means the website is `Blazingly fast`.

## What

There is a button that you can click to call and it calls https://jsonplaceholder.typicode.com/ for fake user data served by a route in our rusty server ... that's it.

### TODO's

- [ ] add some type of cargo to build ui before each `cargo run` and watch files
- [ ] server side generation of our jsx - leverage vite
- [ ] anything to do with a real project or production

I currently don't plan on doing any of these

![Peace out gif](https://media.giphy.com/media/Ru9sjtZ09XOEg/giphy.gif)
