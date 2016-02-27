`arx`
=====

Arx captures a pattern common to Dockerfiles, Travis CI files, simple
deployment scripts and even things like pre-commit from Yelp: executing a
simple specification of a task to run, in terms of commands, URLs and source
code to checkout.

Arx accepts YAML or JSON format documents that describe a task and its
dependencies. The Arx format is simple:

```yaml
code:
  - [unzip, fb.zip]
  - "mv facebook-* fb-sources"                         # Allows shell expansion
  - "cd fb-sources && make all"
  - [nginx, -c, fb-sources/nginx.conf]     # Protects from shell interpretation

data:
  - s3://mega-downloadz/fb.zip
```

but expands to allow for auditing, inlining data and attaching special
environment settings to each sub task:

```yaml
label: deploy-fb

env:
  LANG: en_us.UTF-8

code:
  - [unzip, fb.zip]
  - "mv facebook-* fb-sources"
  - cmd: [make, all]
    pwd: fb-sources
  - cmd: [nginx, -c, nginx.conf]
    pwd: fb-sources

data:
  - source: s3://mega-downloadz/fb.zip
    formerly: git+https://cod3t0wn.onion/repos/fb/one.git
```

Arx also understands some of the richness of archive and URL formats. With
`zip` files (and `tar` files), for example, the fragment identifier `#.` can
be used expand the top-level folder directly in to the present directory, just
like `--strip-components=1`:

```yaml
code:
  - [make, all]
  - [nginx, -c, nginx.conf]

data:
  - zip+s3://mega-downloadz/fb.zip#.
```

(Note that it is an error if there is more than one such folder.)
