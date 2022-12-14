/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */
use crate::models::time_span_seconds::TimeSpanSeconds;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StoreData {
    /// The name of the store
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The absolute url of the store
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The default currency of the store
    #[serde(rename = "defaultCurrency", skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    /// The time after which an invoice is considered expired if not paid. The value will be rounded down to a minute.
    #[serde(rename = "invoiceExpiration", skip_serializing_if = "Option::is_none")]
    pub invoice_expiration: Option<Box<TimeSpanSeconds>>,
    /// The time after which an invoice which has been paid but not confirmed will be considered invalid. The value will be rounded down to a minute.
    #[serde(rename = "monitoringExpiration", skip_serializing_if = "Option::is_none")]
    pub monitoring_expiration: Option<Box<TimeSpanSeconds>>,
    #[serde(rename = "speedPolicy", skip_serializing_if = "Option::is_none")]
    pub speed_policy: Option<crate::models::SpeedPolicy>,
    /// The BOLT11 description of the lightning invoice in the checkout. You can use placeholders '{StoreName}', '{ItemDescription}' and '{OrderId}'.
    #[serde(rename = "lightningDescriptionTemplate", skip_serializing_if = "Option::is_none")]
    pub lightning_description_template: Option<String>,
    /// Consider an invoice fully paid, even if the payment is missing 'x' % of the full amount.
    #[serde(rename = "paymentTolerance", skip_serializing_if = "Option::is_none")]
    pub payment_tolerance: Option<f64>,
    /// If true, then no authentication is needed to create invoices on this store.
    #[serde(rename = "anyoneCanCreateInvoice", skip_serializing_if = "Option::is_none")]
    pub anyone_can_create_invoice: Option<bool>,
    /// If true, the checkout page will ask to enter an email address before accessing payment information.
    #[serde(rename = "requiresRefundEmail", skip_serializing_if = "Option::is_none")]
    pub requires_refund_email: Option<bool>,
    #[serde(rename = "receipt", skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Box<crate::models::ReceiptOptions>>,
    /// If true, lightning payment methods show amount in satoshi in the checkout page.
    #[serde(rename = "lightningAmountInSatoshi", skip_serializing_if = "Option::is_none")]
    pub lightning_amount_in_satoshi: Option<bool>,
    /// Should private route hints be included in the lightning payment of the checkout page.
    #[serde(rename = "lightningPrivateRouteHints", skip_serializing_if = "Option::is_none")]
    pub lightning_private_route_hints: Option<bool>,
    /// Include lightning invoice fallback to on-chain BIP21 payment url.
    #[serde(rename = "onChainWithLnInvoiceFallback", skip_serializing_if = "Option::is_none")]
    pub on_chain_with_ln_invoice_fallback: Option<bool>,
    /// After successfull payment, should the checkout page redirect the user automatically to the redirect URL of the invoice?
    #[serde(rename = "redirectAutomatically", skip_serializing_if = "Option::is_none")]
    pub redirect_automatically: Option<bool>,
    #[serde(rename = "showRecommendedFee", skip_serializing_if = "Option::is_none")]
    pub show_recommended_fee: Option<bool>,
    /// The fee rate recommendation in the checkout page for the on-chain payment to be confirmed after 'x' blocks.
    #[serde(rename = "recommendedFeeBlockTarget", skip_serializing_if = "Option::is_none")]
    pub recommended_fee_block_target: Option<i32>,
    /// The default language to use in the checkout page. (The different translations available are listed [here](https://github.com/btcpayserver/btcpayserver/tree/master/BTCPayServer/wwwroot/locales)
    #[serde(rename = "defaultLang", skip_serializing_if = "Option::is_none")]
    pub default_lang: Option<String>,
    /// URL to a logo to include in the checkout page.
    #[serde(rename = "customLogo", skip_serializing_if = "Option::is_none")]
    pub custom_logo: Option<String>,
    /// URL to a CSS stylesheet to include in the checkout page
    #[serde(rename = "customCSS", skip_serializing_if = "Option::is_none")]
    pub custom_css: Option<String>,
    /// The HTML title of the checkout page (when you over the tab in your browser)
    #[serde(rename = "htmlTitle", skip_serializing_if = "Option::is_none")]
    pub html_title: Option<String>,
    #[serde(rename = "networkFeeMode", skip_serializing_if = "Option::is_none")]
    pub network_fee_mode: Option<crate::models::NetworkFeeMode>,
    /// If true, payjoin will be proposed in the checkout page if possible. ([More information](https://docs.btcpayserver.org/Payjoin/))
    #[serde(rename = "payJoinEnabled", skip_serializing_if = "Option::is_none")]
    pub pay_join_enabled: Option<bool>,
    /// If true, payment methods are enabled individually upon user interaction in the invoice
    #[serde(rename = "lazyPaymentMethods", skip_serializing_if = "Option::is_none")]
    pub lazy_payment_methods: Option<bool>,
    /// The default payment method to load when displaying an invoice. It can be in the format of `BTC_LightningNetwork` to specify Lightning to be the default or `BTC_OnChain`/ `BTC` for on-chain to be the default. 
    #[serde(rename = "defaultPaymentMethod", skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// The id of the store
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StoreData {
    pub fn new() -> StoreData {
        StoreData {
            name: None,
            website: None,
            default_currency: None,
            invoice_expiration: None,
            monitoring_expiration: None,
            speed_policy: None,
            lightning_description_template: None,
            payment_tolerance: None,
            anyone_can_create_invoice: None,
            requires_refund_email: None,
            receipt: None,
            lightning_amount_in_satoshi: None,
            lightning_private_route_hints: None,
            on_chain_with_ln_invoice_fallback: None,
            redirect_automatically: None,
            show_recommended_fee: None,
            recommended_fee_block_target: None,
            default_lang: None,
            custom_logo: None,
            custom_css: None,
            html_title: None,
            network_fee_mode: None,
            pay_join_enabled: None,
            lazy_payment_methods: None,
            default_payment_method: None,
            id: None,
        }
    }
}


