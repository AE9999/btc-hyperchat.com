/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookInvoicePaymentSettledEvent : Callback sent if the `type` is `InvoicePaymentSettled`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookInvoicePaymentSettledEvent {
    /// The delivery id of the webhook
    #[serde(rename = "deliveryId", skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// The id of the webhook
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// If this delivery is a redelivery, the is the delivery id of the original delivery.
    #[serde(rename = "originalDeliveryId", skip_serializing_if = "Option::is_none")]
    pub original_delivery_id: Option<String>,
    /// True if this delivery is a redelivery
    #[serde(rename = "isRedelivery", skip_serializing_if = "Option::is_none")]
    pub is_redelivery: Option<bool>,
    /// The type of this event, current available are `InvoiceCreated`, `InvoiceReceivedPayment`, `InvoiceProcessing`, `InvoiceExpired`, `InvoiceSettled`, `InvoiceInvalid`, and `InvoicePaymentSettled`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The timestamp when this delivery has been created
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f32>,
    /// The store id of the invoice's event
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The invoice id of the invoice's event
    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Whether this payment has been sent after expiration of the invoice
    #[serde(rename = "afterExpiration", skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<bool>,
    /// What payment method was used for this payment
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "payment", skip_serializing_if = "Option::is_none")]
    pub payment: Option<crate::models::Payment>,
}

impl WebhookInvoicePaymentSettledEvent {
    /// Callback sent if the `type` is `InvoicePaymentSettled`
    pub fn new() -> WebhookInvoicePaymentSettledEvent {
        WebhookInvoicePaymentSettledEvent {
            delivery_id: None,
            webhook_id: None,
            original_delivery_id: None,
            is_redelivery: None,
            _type: None,
            timestamp: None,
            store_id: None,
            invoice_id: None,
            after_expiration: None,
            payment_method: None,
            payment: None,
        }
    }
}


