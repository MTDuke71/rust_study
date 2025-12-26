# OperationsApi

All URIs are relative to *http://localhost:8080*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**checkConnected**](#checkconnected) | **GET** /api/v1/unionfind/{id}/connected | Check whether two elements belong to the same connected component. Returns true if they share the same root.|
|[**findElement**](#findelement) | **GET** /api/v1/unionfind/{id}/find | Find the root representative of the set containing the specified element. Uses path compression for O(α(n)) amortized time complexity.|
|[**getStats**](#getstats) | **GET** /api/v1/unionfind/{id}/stats | Retrieve statistics including total number of elements and number of disjoint connected components.|
|[**unionElements**](#unionelements) | **POST** /api/v1/unionfind/{id}/union | Union two elements in the Union-Find instance, merging their sets if they are not already connected.|

# **checkConnected**
> ConnectedResponse checkConnected()

Determines connectivity by comparing the root representatives of both elements.

### Example

```typescript
import {
    OperationsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OperationsApi(configuration);

let id: string; //Instance ID (default to undefined)
let element1: number; //First element (default to undefined)
let element2: number; //Second element (default to undefined)

const { status, data } = await apiInstance.checkConnected(
    id,
    element1,
    element2
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Instance ID | defaults to undefined|
| **element1** | [**number**] | First element | defaults to undefined|
| **element2** | [**number**] | Second element | defaults to undefined|


### Return type

**ConnectedResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Connectivity check successful |  -  |
|**400** | Invalid element indices |  -  |
|**404** | Instance not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **findElement**
> FindResponse findElement()

Returns the canonical representative of the set, applying path compression to optimize future queries.

### Example

```typescript
import {
    OperationsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OperationsApi(configuration);

let id: string; //Instance ID (default to undefined)
let element: number; //Element to find root for (default to undefined)

const { status, data } = await apiInstance.findElement(
    id,
    element
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Instance ID | defaults to undefined|
| **element** | [**number**] | Element to find root for | defaults to undefined|


### Return type

**FindResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Find operation successful |  -  |
|**400** | Invalid element index |  -  |
|**404** | Instance not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStats**
> StatsResponse getStats()

Returns structural information about the current state of the Union-Find instance.

### Example

```typescript
import {
    OperationsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OperationsApi(configuration);

let id: string; //Instance ID (default to undefined)

const { status, data } = await apiInstance.getStats(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**string**] | Instance ID | defaults to undefined|


### Return type

**StatsResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Statistics retrieved successfully |  -  |
|**404** | Instance not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **unionElements**
> UnionResponse unionElements(unionRequest)

Connects two elements by merging their representative sets using union-by-rank optimization.

### Example

```typescript
import {
    OperationsApi,
    Configuration,
    UnionRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OperationsApi(configuration);

let id: string; //Instance ID (default to undefined)
let unionRequest: UnionRequest; //

const { status, data } = await apiInstance.unionElements(
    id,
    unionRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **unionRequest** | **UnionRequest**|  | |
| **id** | [**string**] | Instance ID | defaults to undefined|


### Return type

**UnionResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Union operation successful |  -  |
|**400** | Invalid element indices |  -  |
|**404** | Instance not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

