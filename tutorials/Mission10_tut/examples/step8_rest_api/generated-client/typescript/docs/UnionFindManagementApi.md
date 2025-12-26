# UnionFindManagementApi

All URIs are relative to *http://localhost:8080*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createInstance**](#createinstance) | **POST** /api/v1/unionfind | Create a new Union-Find instance with the specified number of elements. Each element starts in its own separate set.|
|[**deleteInstance**](#deleteinstance) | **DELETE** /api/v1/unionfind/{id} | Permanently delete a Union-Find instance and free its resources. This operation cannot be undone.|

# **createInstance**
> CreateResponse createInstance(createRequest)

Creates an isolated disjoint-set data structure with the specified capacity.

### Example

```typescript
import {
    UnionFindManagementApi,
    Configuration,
    CreateRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new UnionFindManagementApi(configuration);

let createRequest: CreateRequest; //

const { status, data } = await apiInstance.createInstance(
    createRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createRequest** | **CreateRequest**|  | |


### Return type

**CreateResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Instance created successfully |  -  |
|**400** | Invalid request parameters |  -  |
|**422** | Unprocessable Entity - JSON deserialization failed |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteInstance**
> deleteInstance()

Removes the instance from memory. All subsequent operations on this ID will fail.

### Example

```typescript
import {
    UnionFindManagementApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UnionFindManagementApi(configuration);

let id: string; //Instance ID (default to undefined)

const { status, data } = await apiInstance.deleteInstance(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Instance ID | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[api_key](../README.md#api_key), [bearer_auth](../README.md#bearer_auth), [oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**204** | Instance deleted successfully |  -  |
|**401** | Unauthorized - Missing or invalid credentials |  -  |
|**403** | Forbidden - Insufficient permissions |  -  |
|**404** | Instance not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

