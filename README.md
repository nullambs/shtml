# SHTML

> ... CSS-ish HTML..

---

have you ever wondered **why HTML** is so painful to write by hand? wellp, me too. That's why i created this preprocessor which lets you writ CSS-C-like HTML which then gets rendered in DOM

## features

> this is kind of a whichlist of what i would want it to and hope it does... it's not vibe-coded i swear

---

- WASM-only! it means, now instead of transferring plain HTML you can send SHTML which will then get rendered on the client side!
- partially full compaitibility with plain HTML, HTMLX.. you may even **try** to use __vue__ or __angular__...

## usage

> use wisely

---

see [examples](examples) folder

~ BTW, shtml.js also exports a `parse` function which takes a **SHTML** string and an element to attach the contents to.. in case you don't like the `shtml` attribute.

---

## syntax

> this may be of interest to you based on the fact you've come so far

---

general syntax is the following: tag#id.class1.class2 [ attr1="attr1 value" ... ] { ... }

1. **tag** is mandatory, name accepts letters, digidts, hyphens... that should be enough
2. **id** is optional, starts with `#`, always comes before classes
3. **classes** are optional, start with `.`, always come after **id**
4. **attributes** are optional, surrounded by `[ ... ]`, separated by __space__
5. **inner SHTML** is mandatory (even for void elements, eg. hr, br), surrounded by `{ ... }`

**text nodes** are special elements, created by writing `" ... "` with the text in place of `...`

thanks,,

