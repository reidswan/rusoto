#![cfg(feature = "ds")]

extern crate rusoto_core;
extern crate rusoto_ds;

use rusoto_core::{Region, RusotoError};
use rusoto_ds::{
    DescribeDirectoriesRequest, DescribeTrustsRequest, DirectoryService, DirectoryServiceClient, DescribeEventTopicsRequest, DescribeConditionalForwardersRequest, DescribeDomainControllersRequest, DescribeConditionalForwardersError, DescribeDomainControllersError,
};

#[test]
fn should_describe_trusts() {
    let client = DirectoryServiceClient::new(Region::UsWest2);
    let request = DescribeTrustsRequest::default();

    client.describe_trusts(request).sync().unwrap();
}

#[test]
fn should_describe_directories() {
    let client = DirectoryServiceClient::new(Region::UsWest2);
    let request = DescribeDirectoriesRequest::default();

    client.describe_directories(request).sync().unwrap();
}

#[test]
fn should_conditional_forwarders() {
    let client = DirectoryServiceClient::new(Region::UsWest2);
    let mut request = DescribeConditionalForwardersRequest::default();
	request.directory_id = "d-11111aaaaa".to_string();

    match client.describe_conditional_forwarders(request).sync() {
		  Err(RusotoError::Service(DescribeConditionalForwardersError::EntityDoesNotExist(msg))) => {
            assert!(msg.contains("does not exist."))
        }
        err @ _ => panic!("Expected EntityDoesNotExist error, got {:#?}", err)
	}
}

#[test]
fn should_describe_domain_controllers() {
    let client = DirectoryServiceClient::new(Region::UsWest2);
    let mut request = DescribeDomainControllersRequest::default();
	request.directory_id = "d-11111aaaaa".to_string();

    match client.describe_domain_controllers(request).sync() {
		Err(RusotoError::Service(DescribeDomainControllersError::EntityDoesNotExist(msg))) => {
            assert!(msg.contains("does not exist."))
        }
        err @ _ => panic!("Expected EntityDoesNotExist error, got {:#?}", err)
	}
}

#[test]
fn should_describe_event_topics() {
    let client = DirectoryServiceClient::new(Region::UsWest2);
    let request = DescribeEventTopicsRequest::default();

    client.describe_event_topics(request).sync().unwrap();
}
