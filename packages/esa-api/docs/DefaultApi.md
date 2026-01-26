# \DefaultApi

All URIs are relative to *https://api.esa.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_teams_team_name_posts_get**](DefaultApi.md#v1_teams_team_name_posts_get) | **get** /v1/teams/{team_name}/posts | List posts in a team
[**v1_teams_team_name_posts_post_number_get**](DefaultApi.md#v1_teams_team_name_posts_post_number_get) | **get** /v1/teams/{team_name}/posts/{post_number} | Get a post
[**v1_teams_team_name_posts_post_number_star_delete**](DefaultApi.md#v1_teams_team_name_posts_post_number_star_delete) | **delete** /v1/teams/{team_name}/posts/{post_number}/star | Unstar a post
[**v1_teams_team_name_posts_post_number_star_post**](DefaultApi.md#v1_teams_team_name_posts_post_number_star_post) | **post** /v1/teams/{team_name}/posts/{post_number}/star | Star a post
[**v1_teams_team_name_posts_post_number_watch_delete**](DefaultApi.md#v1_teams_team_name_posts_post_number_watch_delete) | **delete** /v1/teams/{team_name}/posts/{post_number}/watch | Unwatch a post
[**v1_teams_team_name_posts_post_number_watch_post**](DefaultApi.md#v1_teams_team_name_posts_post_number_watch_post) | **post** /v1/teams/{team_name}/posts/{post_number}/watch | Watch a post
[**v1_user_get**](DefaultApi.md#v1_user_get) | **get** /v1/user | Get current authenticated user



## v1_teams_team_name_posts_get

> crate::models::PostListResponse v1_teams_team_name_posts_get(team_name, q, include, page, sort, order)
List posts in a team

Returns a list of posts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** |  | [required] |
**q** | Option<**String**> | Filter condition for posts. |  |
**include** | Option<**String**> | Include related resources (comments, comments.stargazers, stargazers). |  |
**page** | Option<**i32**> | 取得するページ |  |
**sort** | Option<**String**> | Sort key. |  |[default to updated]
**order** | Option<**String**> | Sort order. |  |[default to desc]

### Return type

[**crate::models::PostListResponse**](PostListResponse.md)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_teams_team_name_posts_post_number_get

> crate::models::Post v1_teams_team_name_posts_post_number_get(team_name, post_number, include)
Get a post

Returns the specified post.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** |  | [required] |
**post_number** | **i32** |  | [required] |
**include** | Option<**String**> | Include related resources (comments, comments.stargazers, stargazers). |  |

### Return type

[**crate::models::Post**](Post.md)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_teams_team_name_posts_post_number_star_delete

> v1_teams_team_name_posts_post_number_star_delete(team_name, post_number)
Unstar a post

Removes your star from the post.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** |  | [required] |
**post_number** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_teams_team_name_posts_post_number_star_post

> v1_teams_team_name_posts_post_number_star_post(team_name, post_number, inline_object)
Star a post

Adds a star to the post. You can include a message body with the star.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** |  | [required] |
**post_number** | **i32** |  | [required] |
**inline_object** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_teams_team_name_posts_post_number_watch_delete

> v1_teams_team_name_posts_post_number_watch_delete(team_name, post_number)
Unwatch a post

Removes the post from the authenticated user's watch list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** |  | [required] |
**post_number** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_teams_team_name_posts_post_number_watch_post

> v1_teams_team_name_posts_post_number_watch_post(team_name, post_number)
Watch a post

Adds the post to the authenticated user's watch list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** |  | [required] |
**post_number** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_user_get

> crate::models::User v1_user_get(include)
Get current authenticated user

Returns the user associated with the access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | Include related resources. Use \"teams\" to include the teams array. |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[accessTokenQuery](../README.md#accessTokenQuery), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

