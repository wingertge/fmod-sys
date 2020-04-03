# fmod-sys

Bindgen generated wrappers for FMOD

This library is aiming to be an idiomatic "-sys" version of wrappers for [FMOD](https://fmod.com).

The library itself is licensed using the [MIT license](./LICENSE), but the `fmodapi` folder contains headers from the FMOD API download, and those files are licensed under the [FMOD License](./fmodapi/LICENSE.TXT).

# Usage

While I'm still getting this testing, you can use the `git` feature of Cargo.toml dependencies to use this crate:

```toml
[dependencies]
fmod-sys = { git = "https://github.com/khonsulabs/fmod-sys.git" }
```

You will need to put the fmod libraries in an appropriate location for linking to succeed.

# Building

To update the headers, just copy them into the fmodapi folder, make sure the LICENSE.TXT doesn't need to be updated, and then the build.rs script should automatically generate the new bindings.

## Mac OS Notes

When attempting to run an application linking against libfmod.dylib without signing it, you will be prompted for a security warning. Find the file in the Finder, right click on it and choose Open. It makes no sense to do this, but it will prompt you just like it would an unsigned application -- Click open and it will launch the Terminal app and do nothing.

However, once you've done this step, you can run the application until you replace the library wth a new version.

# Warning

![I don't know what I'm doing meme](https://media.giphy.com/media/xDQ3Oql1BN54c/giphy.gif)

This is my first bindgen crate. I have experience using other bindgen wrappers like the `gl` crate, but this is my first attempt at bundling a crate for others. Please don't hesitate to leave constructive feedback as issues or submit your own pull requests!

## Other notes

I plan on writing a rusty-wrapper for FMOD Studio. More details to come.
