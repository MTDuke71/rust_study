# ErrorResponse

Standard error response structure

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **string** | Machine-readable error code | [default to undefined]
**details** | **{ [key: string]: any; }** | Optional additional error details | [optional] [default to undefined]
**field_errors** | **{ [key: string]: string; }** | Optional field-level validation errors | [optional] [default to undefined]
**message** | **string** | Human-readable error message | [default to undefined]

## Example

```typescript
import { ErrorResponse } from './api';

const instance: ErrorResponse = {
    code,
    details,
    field_errors,
    message,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
