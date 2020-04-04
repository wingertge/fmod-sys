# fmodapi directory information

The files contained in this folder are licensed under the [FMOD END USER LICENCE AGREEMENT](./LICENSE.txt). They are from the Linux x86-64 FMOD Studio API download located on [fmod.com](https://fmod.com).

The headers are used for generating bindings. The libraries are only used in the CI process. Your own version of the libraries are what is linked against when building.

The version of the headers is from `2.00.08`.

To update to a new version, download the Linux x86-64 Studio API package and copy the files like this:

```bash
cp -r PATH_TO_SDK/api/core/lib/x86_64/* fmodapi/libs/
cp -r PATH_TO_SDK/api/studio/lib/x86_64/* fmodapi/libs/
cp -r PATH_TO_SDK/api/core/inc/*.h fmodapi/
cp -r PATH_TO_SDK/api/studio/inc/*.h fmodapi/
```
