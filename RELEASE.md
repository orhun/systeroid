# Creating a Release

1. Update [CHANGELOG.md](./CHANGELOG.md)
    - Do not commit changes yet.
2. Run the [release script](./scripts/release.sh): `./scripts/release.sh v[X.Y.Z]`
   - Changes will be committed and a tag will be created.
3. Push the changes: `git push`
   - Check if [Continuous Integration](https://github.com/orhun/systeroid/actions) workflow is completed successfully before moving to the next step.
4. Push the tags: `git push --tags`
   - [GitHub](https://github.com/orhun/systeroid/releases), [crates.io](https://crates.io/crates/systeroid/), and [Docker Hub](https://hub.docker.com/r/orhunp/systeroid) releases are automated via [GitHub actions](./.github/workflows/cd.yml) and triggered by pushing a tag.
5. Check the status of [Continuous Deployment](https://github.com/orhun/systeroid/actions) workflow.
