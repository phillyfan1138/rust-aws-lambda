use custom_serde::*;
use serde_json::Value;
use std::collections::HashMap;

/// `ApiGatewayProxyRequest` contains data coming from the API Gateway proxy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyRequest {
    /// The resource path defined in API Gateway
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    /// The url path for the caller
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayProxyRequestContext,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayProxyResponse` configures the response to be returned by API Gateway for the request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayProxyRequestContext` contains the information to identify the AWS account and resources invoking the
/// Lambda function. It also includes Cognito identity information for the caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyRequestContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub identity: ApiGatewayRequestIdentity,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub authorizer: HashMap<String, Value>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    /// The API Gateway rest API Id
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
}

/// `ApiGatewayRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayRequestIdentity {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub caller: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userArn")]
    pub user_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user: Option<String>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequestIdentity {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
}

/// `ApiGatewayCustomAuthorizerContext` represents the expected format of an API Gateway custom authorizer response.
/// Deprecated. Code should be updated to use the Authorizer map from APIGatewayRequestIdentity. Ex: Authorizer["principalId"]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerContext {
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "stringKey")]
    pub string_key: Option<String>,
    #[serde(rename = "numKey")]
    pub num_key: Option<i64>,
    #[serde(rename = "boolKey")]
    pub bool_key: Option<bool>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequestContext` represents the expected format of an API Gateway custom authorizer response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequestContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub identity: ApiGatewayCustomAuthorizerRequestTypeRequestIdentity,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
}

/// `ApiGatewayCustomAuthorizerRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "authorizationToken")]
    pub authorization_token: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: Option<String>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayCustomAuthorizerRequestTypeRequestContext,
}

/// `ApiGatewayCustomAuthorizerResponse` represents the expected format of an API Gateway authorization response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerResponse {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "policyDocument")]
    pub policy_document: ApiGatewayCustomAuthorizerPolicy,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub context: HashMap<String, Value>,
    #[serde(rename = "usageIdentifierKey")]
    pub usage_identifier_key: Option<String>,
}

/// `ApiGatewayCustomAuthorizerPolicy` represents an IAM policy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerPolicy {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    pub statement: Vec<IamPolicyStatement>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IamPolicyStatement {
    pub action: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub effect: Option<String>,
    pub resource: Vec<String>,
}
