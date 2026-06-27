# TODO - Issue #28 revocation-registry issuer bypass

- [ ] Update `contracts/revocation-registry/src/lib.rs` to lock revocation authority per `vc_hash` (first issuer registers; later issuer must match)
- [ ] Add/extend revocation-registry tests (unit tests inside the contract) for “only original/registered issuer can revoke”
- [ ] Add integration test in `contracts/tests/src/integration_test.rs` covering issuer bypass prevention
- [ ] Update `docs/architecture.md` to document the authority model
- [ ] Run workspace tests to ensure snapshots and all tests pass

