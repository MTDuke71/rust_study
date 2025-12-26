# CreateRequest

Request body for creating a new Union-Find instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | **number** | Number of elements in the set (must be &gt; 0)  Common use cases: - 10: Small network - 100: Medium network or 10x10 percolation grid - 1000: Large network - 1920: HD image segments  Larger values may impact performance. | [default to undefined]

## Example

```typescript
import { CreateRequest } from './api';

const instance: CreateRequest = {
    size,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
