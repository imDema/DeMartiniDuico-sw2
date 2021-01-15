

## Documentation for API Endpoints


Class | HTTP request | Description
------------ | ------------- | -------------
*AccountApi* | **GET** /register/confirm | Submit email confirmation
*AccountApi* | **POST** /login | Logs in and returns the authentication  cookie
*AccountApi* | **POST** /register | Request creation of a new account
*BookingApi* | **GET** /shop/{shop_id}/booking/availability | Get information about the time slot availability for bookings
*BookingApi* | **POST** /shop/{shop_id}/booking/new | Request a booking for a shop
*BookingApi* | **GET** /tokens | Get active tokens for the customer
*ManageApi* | **POST** /staff/manage/shop/add | Add a new shop to the database
*ManageApi* | **POST** /staff/manage/shop/edit/{shop_id} | Edit an existing shop listing
*ManageApi* | **POST** /staff/manage/shop/hide/{shop_id} | Make shop private and hidden from customers
*ManageApi* | **POST** /staff/manage/shop/list | Get a list of all managed shops
*ManageApi* | **POST** /staff/manage/shop/show/{shop_id} | Make shop public and reachable from customers
*ShopApi* | **GET** /search | Search for a shop by name
*ShopApi* | **GET** /shop/{shop_id} | Get available shop information
*StaffApi* | **GET** /staff/shop/{shop_id}/booking/list | Get detailed information about the current and future bookings
*StaffApi* | **POST** /staff/shop/{shop_id}/ticket/new-substitute | Request creation of a substitute ticket
*StaffApi* | **GET** /staff/shop/{shop_id}/ticket/queue | Get detailed information about the current queue status
*StaffApi* | **GET** /staff/shop/{shop_id}/token/info | Get token information and validity
*StaffApi* | **POST** /staff/shop/{shop_id}/token/log-entry | Log entry, consume the token and update shop occupancy information
*StaffApi* | **POST** /staff/shop/{shop_id}/token/log-exit | Log exit, update shop occupancy information
*StaffAccountApi* | **POST** /staff/login | Log in and return authentication cookie
*StaffAccountApi* | **POST** /staff/register | Request the creation of a new staff account
*TicketApi* | **POST** /shop/{shop_id}/ticket/new | Request a ticket for a shop
*TicketApi* | **GET** /shop/{shop_id}/ticket/queue | Get information about the queue status and approximate waiting time
*TicketApi* | **GET** /ticket/est | Get the estimated waiting time for a ticket
*TicketApi* | **GET** /tokens | Get active tokens for the customer


## Documentation for Models

 - [Department](./Models/Department.md)
 - [InlineObject](./Models/InlineObject.md)
 - [QueueEst](./Models/QueueEst.md)
 - [RequestBooking](./Models/RequestBooking.md)
 - [RequestLogin](./Models/RequestLogin.md)
 - [RequestRegister](./Models/RequestRegister.md)
 - [RequestTicket](./Models/RequestTicket.md)
 - [SearchResult](./Models/SearchResult.md)
 - [Shop](./Models/Shop.md)
 - [ShopSchedule](./Models/ShopSchedule.md)
 - [TimeSpan](./Models/TimeSpan.md)
 - [TokenBooking](./Models/TokenBooking.md)
 - [TokenCode](./Models/TokenCode.md)
 - [TokenTicket](./Models/TokenTicket.md)
 - [Tokens](./Models/Tokens.md)


## Documentation for Authorization

### customerAuth

- **Type**: API key
- **API key parameter name**: SESSIONID


### staffAuth

- **Type**: API key
- **API key parameter name**: SESSIONID


#### **confirmRegistration**
> confirmRegistration(code)

Submit email confirmation

    Finalizes account creation

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **code** | **String**| Confirmation code received by email 

###### Authorization

No authorization required

#### **loginPost**
> loginPost(RequestLogin)

Logs in and returns the authentication  cookie

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **RequestLogin** | [**RequestLogin**](../Models/RequestLogin.md)| A JSON object containing the login and password. 

###### Authorization

No authorization required

#### **registerPost**
> registerPost(RequestRegister)

Request creation of a new account

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **RequestRegister** | [**RequestRegister**](../Models/RequestRegister.md)| Credentials 

###### Authorization

No authorization required


#### **shopShopIdBookingAvailabilityGet**
> List shopShopIdBookingAvailabilityGet(shop\_id, day)

Get information about the time slot availability for bookings

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **day** | **date**|  

###### Return type

[**List**](../Models/object.md)

###### Authorization

No authorization required

#### **shopShopIdBookingNewPost**
> TokenBooking shopShopIdBookingNewPost(shop\_id, RequestBooking)

Request a booking for a shop

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **RequestBooking** | [**RequestBooking**](../Models/RequestBooking.md)|  

###### Return type

[**TokenBooking**](../Models/TokenBooking.md)

###### Authorization

[customerAuth](../README.md#customerAuth)

#### **tokensGet**
> Tokens tokensGet()

Get active tokens for the customer


###### Return type

[**Tokens**](../Models/Tokens.md)

###### Authorization

[customerAuth](../README.md#customerAuth)


#### **staffManageShopAddPost**
> Shop staffManageShopAddPost(InlineObject)

Add a new shop to the database

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **InlineObject** | [**InlineObject**](../Models/InlineObject.md)|  

###### Return type

[**Shop**](../Models/Shop.md)

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffManageShopEditShopIdPost**
> staffManageShopEditShopIdPost(shop\_id, Shop)

Edit an existing shop listing

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **Shop** | [**Shop**](../Models/Shop.md)|  

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffManageShopHideShopIdPost**
> staffManageShopHideShopIdPost(shop\_id)

Make shop private and hidden from customers

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffManageShopListPost**
> List staffManageShopListPost()

Get a list of all managed shops


###### Return type

[**List**](../Models/SearchResult.md)

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffManageShopShowShopIdPost**
> staffManageShopShowShopIdPost(shop\_id)

Make shop public and reachable from customers

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  

###### Authorization

[staffAuth](../README.md#staffAuth)


#### **searchGet**
> List searchGet(q)

Search for a shop by name

    Search for a shop by name

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **q** | **String**| Search string 

###### Return type

[**List**](../Models/SearchResult.md)

###### Authorization

No authorization required

#### **shopShopIdGet**
> Shop shopShopIdGet(shop\_id)

Get available shop information

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  

###### Return type

[**Shop**](../Models/Shop.md)

###### Authorization

No authorization required


#### **staffLoginPost**
> staffLoginPost(RequestLogin)

Log in and return authentication cookie

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **RequestLogin** | [**RequestLogin**](../Models/RequestLogin.md)| A JSON object containing the login and password. 

###### Authorization

No authorization required

#### **staffRegisterPost**
> staffRegisterPost()

Request the creation of a new staff account


###### Authorization

No authorization required


#### **staffShopShopIdBookingListGet**
> List staffShopShopIdBookingListGet(shop\_id)

Get detailed information about the current and future bookings

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  

###### Return type

[**List**](../Models/TokenBooking.md)

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffShopShopIdTicketNewSubstitutePost**
> TokenTicket staffShopShopIdTicketNewSubstitutePost(shop\_id, RequestTicket)

Request creation of a substitute ticket

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **RequestTicket** | [**RequestTicket**](../Models/RequestTicket.md)|  

###### Return type

[**TokenTicket**](../Models/TokenTicket.md)

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffShopShopIdTicketQueueGet**
> List staffShopShopIdTicketQueueGet(shop\_id)

Get detailed information about the current queue status

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  

###### Return type

[**List**](../Models/TokenTicket.md)

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffShopShopIdTokenInfoGet**
> TokenTicket staffShopShopIdTokenInfoGet(shop\_id, uid)

Get token information and validity

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **uid** | **String**|  

###### Return type

[**TokenTicket**](../Models/TokenTicket.md)

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffShopShopIdTokenLogEntryPost**
> staffShopShopIdTokenLogEntryPost(shop\_id, TokenCode)

Log entry, consume the token and update shop occupancy information

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **TokenCode** | [**TokenCode**](../Models/TokenCode.md)|  

###### Authorization

[staffAuth](../README.md#staffAuth)

#### **staffShopShopIdTokenLogExitPost**
> staffShopShopIdTokenLogExitPost(shop\_id, TokenCode)

Log exit, update shop occupancy information

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **TokenCode** | [**TokenCode**](../Models/TokenCode.md)|  

###### Authorization

[staffAuth](../README.md#staffAuth)


#### **shopShopIdTicketNewPost**
> TokenTicket shopShopIdTicketNewPost(shop\_id, RequestTicket)

Request a ticket for a shop

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  
 **RequestTicket** | [**RequestTicket**](../Models/RequestTicket.md)|  

###### Return type

[**TokenTicket**](../Models/TokenTicket.md)

###### Authorization

[customerAuth](../README.md#customerAuth)

#### **shopShopIdTicketQueueGet**
> QueueEst shopShopIdTicketQueueGet(shop\_id)

Get information about the queue status and approximate waiting time

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **shop\_id** | **String**|  

###### Return type

[**QueueEst**](../Models/QueueEst.md)

###### Authorization

No authorization required

#### **ticketEstGet**
> QueueEst ticketEstGet(uid)

Get the estimated waiting time for a ticket

###### Parameters

Name | Type | Description  
------------- | ------------- | ------------- 
 **uid** | **String**| Ticket unique identifier 

###### Return type

[**QueueEst**](../Models/QueueEst.md)

###### Authorization

[customerAuth](../README.md#customerAuth)

#### **tokensGet**
> Tokens tokensGet()

Get active tokens for the customer


###### Return type

[**Tokens**](../Models/Tokens.md)

###### Authorization

[customerAuth](../README.md#customerAuth)

#### Department
##### Properties

Name | Type 
------------ | ------------- 
**uid** | [**String**](string.md) 
**description** | [**String**](string.md) 
**capacity** | [**Integer**](integer.md) 


#### InlineObject
##### Properties

Name | Type 
------------ | ------------- 
**name** | [**String**](string.md) 


#### QueueEst
##### Properties

Name | Type 
------------ | ------------- 
**people** | [**Integer**](integer.md) 
**est** | [**Date**](DateTime.md) 


#### RequestBooking
##### Properties

Name | Type 
------------ | ------------- 
**shop\_id** | [**String**](string.md) 
**department\_ids** | [**List**](string.md) 
**start\_time** | [**Date**](DateTime.md) 
**end\_time** | [**Date**](DateTime.md) 


#### RequestLogin
##### Properties

Name | Type 
------------ | ------------- 
**email** | [**String**](string.md) 
**password** | [**String**](string.md) 
**remember** | [**Boolean**](boolean.md) 


#### RequestRegister
##### Properties

Name | Type 
------------ | ------------- 
**email** | [**String**](string.md) 
**password** | [**String**](string.md) 


#### RequestTicket
##### Properties

Name | Type 
------------ | ------------- 
**shop\_id** | [**String**](string.md) 
**department\_ids** | [**List**](string.md) 


#### SearchResult
##### Properties

Name | Type 
------------ | ------------- 
**uid** | [**String**](string.md) 
**name** | [**String**](string.md) 
**image** | [**String**](string.md) 
**description** | [**String**](string.md) 


#### Shop
##### Properties

Name | Type 
------------ | ------------- 
**uid** | [**String**](string.md) 
**name** | [**String**](string.md) 
**description** | [**String**](string.md) 
**image** | [**String**](string.md) 
**location** | [**String**](string.md) 
**departments** | [**List**](Department.md) 
**schedule** | [**Shop_schedule**](Shop_schedule.md) 


#### ShopSchedule
##### Properties

Name | Type 
------------ | ------------- 
**mo** | [**List**](TimeSpan.md) 
**tu** | [**List**](TimeSpan.md) 
**we** | [**List**](TimeSpan.md) 
**th** | [**List**](TimeSpan.md) 
**fr** | [**List**](TimeSpan.md) 
**sa** | [**List**](TimeSpan.md) 
**su** | [**List**](TimeSpan.md) 


#### TimeSpan
##### Properties

Name | Type 
------------ | ------------- 
**start** | [**String**](string.md) 
**end** | [**String**](string.md) 


#### TokenBooking
##### Properties

Name | Type 
------------ | ------------- 
**uid** | [**String**](string.md) 
**shop\_id** | [**String**](string.md) 
**department\_ids** | [**List**](string.md) 
**start\_time** | [**Date**](DateTime.md) 
**end\_time** | [**Date**](DateTime.md) 


#### TokenCode
##### Properties

Name | Type 
------------ | ------------- 
**uid** | [**String**](string.md) 


#### Tokens
##### Properties

Name | Type 
------------ | ------------- 
**tickets** | [**List**](TokenTicket.md) 
**bookings** | [**List**](TokenBooking.md) 


#### TokenTicket
##### Properties

Name | Type 
------------ | ------------- 
**uid** | [**String**](string.md) 
**shop\_id** | [**String**](string.md) 
**department\_ids** | [**List**](string.md) 
**creation** | [**Date**](DateTime.md) 
**expiration** | [**Date**](DateTime.md) 
**state** | [**String**](string.md) 


