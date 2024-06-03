# Leader Determination Criteria in is_leader Function

The criteria for being the leader in the context of the is_leader function are based on the current session index and the list of validators. Here are the steps and criteria for determining if a node is the leader:
Steps
1. Fetch the Current Set of Validators

The function retrieves the list of validators from the validator_set pallet. This list contains the account identifiers of all validators in the current session.

```

let validators = validator_set::Validators::<T>::get();

```

2. Get the Current Session Index

The function gets the current session index from the pallet_session. The session index is a number that increments with each new session.

```

let current_index = pallet_session::Pallet::<T>::current_index();


```
3. Calculate the Leader

The function determines which validator is the leader for the current session by using the modulus operation on the session index and the number of validators. This calculation ensures that each validator gets a turn to be the leader in a round-robin manner.

```

if let Some(session_leader) = validators.get(current_index as usize % validators.len()) {
    // ...
}


```
4. Convert ValidatorId to AuthorityId

The function converts the ValidatorId of the calculated leader to an AuthorityId. The AuthorityId is used to match against the public keys that the node owns.

```

let leader = Self::convert_session_validator_id_to_pallet_validator_id(session_leader.clone());

if let Ok(leader_authority_id) = Self::convert_validator_id_to_authority_id(leader) {
    // ...
}

```
5. Fetch Local Keys

The function retrieves the public keys that the current node owns. These keys are used to verify if the node is the leader.

```

let local_keys = Self::fetch_local_keys();


```
6. Check if the Node is the Leader

The function compares the leader's AuthorityId with the local keys. If any of the local keys match the leader's AuthorityId, it means the current node is the leader.

```

for local_key in local_keys {
    if local_key == leader_authority_id {
        return true;
    }
}

```
## Summary

### In summary, a node is considered the leader if:

    1. It is listed in the current set of validators.
    2. The current session index, when taken modulo the number of validators, points to this node as the leader.
    3. The AuthorityId corresponding to the leader matches one of the public keys that the node owns.

This approach ensures that leadership rotates among the validators in a predictable and fair manner, based on the session index.