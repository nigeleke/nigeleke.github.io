# Nigel Eke - Personal Site

## Build

```bash
$ export NIXPKGS_ALLOW_UNFREE=1
$ nix develop --impure
$ zola build
```

## Run Locally

```bash
$ zola serve
```
Browse [http://localhost:1111](http://127.0.0.1:1111/)

## Publish

GitHub workflow `deploy.yml`
