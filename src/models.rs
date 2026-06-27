#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionBillingAddress {
    pub country: Box<crate::models::CountryCode>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    pub zipcode: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionCustomization {
    pub force_language: Option<String>,
    pub show_on_demand_tag: Option<bool>,
    pub show_order_details: Option<bool>,
    pub theme: Option<String>,
    pub theme_config: Option<Box<crate::models::ThemeConfig>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionFlags {
    pub allow_currency_selection: Option<bool>,
    pub allow_customer_editing_business_name: Option<bool>,
    pub allow_customer_editing_city: Option<bool>,
    pub allow_customer_editing_country: Option<bool>,
    pub allow_customer_editing_email: Option<bool>,
    pub allow_customer_editing_name: Option<bool>,
    pub allow_customer_editing_state: Option<bool>,
    pub allow_customer_editing_street: Option<bool>,
    pub allow_customer_editing_tax_id: Option<bool>,
    pub allow_customer_editing_zipcode: Option<bool>,
    pub allow_discount_code: Option<bool>,
    pub allow_editing_addons: Option<bool>,
    pub allow_phone_number_collection: Option<bool>,
    pub allow_tax_id: Option<bool>,
    pub always_create_new_customer: Option<bool>,
    pub redirect_immediately: Option<bool>,
    pub require_phone_number: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionRequest {
    pub product_cart: Vec<crate::models::ProductItemReq>,
    pub allowed_payment_method_types: Option<Vec<crate::models::PaymentMethodTypes>>,
    pub billing_address: Option<Box<crate::models::CheckoutSessionBillingAddress>>,
    pub billing_currency: Option<Box<crate::models::Currency>>,
    pub cancel_url: Option<String>,
    pub confirm: Option<bool>,
    pub custom_fields: Option<Vec<crate::models::CustomField>>,
    pub customer: Option<Box<crate::models::CustomerRequest>>,
    pub customer_business_name: Option<String>,
    pub customization: Option<Box<crate::models::CheckoutSessionCustomization>>,
    pub discount_code: Option<String>,
    pub discount_codes: Option<Vec<String>>,
    pub feature_flags: Option<Box<crate::models::CheckoutSessionFlags>>,
    pub force_3ds: Option<bool>,
    pub mandate_min_amount_inr_paise: Option<i64>,
    pub metadata: Option<Box<crate::models::Metadata>>,
    pub minimal_address: Option<bool>,
    pub payment_method_id: Option<String>,
    pub product_collection_id: Option<String>,
    pub return_url: Option<String>,
    pub short_link: Option<bool>,
    pub show_saved_payment_methods: Option<bool>,
    pub subscription_data: Option<Box<crate::models::SubscriptionData>>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionResponse {
    pub session_id: String,
    pub checkout_url: Option<String>,
    pub client_secret: Option<String>,
    pub payment_id: Option<String>,
    pub publishable_key: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionStatus {
    pub id: String,
    pub created_at: String,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub payment_id: Option<String>,
    pub payment_status: Option<Box<crate::models::IntentStatus>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomField {
    pub field_type: String,
    pub key: String,
    pub label: String,
    pub options: Option<Vec<String>>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductItemReq {
    pub product_id: String,
    pub quantity: i64,
    pub addons: Option<Vec<crate::models::AttachAddon>>,
    pub amount: Option<i64>,
    pub credit_entitlements: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionData {
    pub on_demand: Option<Box<crate::models::OnDemandSubscription>>,
    pub trial_period_days: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeConfig {
    pub dark: Option<Box<crate::models::ThemeModeConfig>>,
    pub font_primary_url: Option<String>,
    pub font_secondary_url: Option<String>,
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub light: Option<Box<crate::models::ThemeModeConfig>>,
    pub pay_button_text: Option<String>,
    pub radius: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeModeConfig {
    pub bg_primary: Option<String>,
    pub bg_secondary: Option<String>,
    pub border_primary: Option<String>,
    pub border_secondary: Option<String>,
    pub button_primary: Option<String>,
    pub button_primary_hover: Option<String>,
    pub button_secondary: Option<String>,
    pub button_secondary_hover: Option<String>,
    pub button_text_primary: Option<String>,
    pub button_text_secondary: Option<String>,
    pub input_focus_border: Option<String>,
    pub text_error: Option<String>,
    pub text_placeholder: Option<String>,
    pub text_primary: Option<String>,
    pub text_secondary: Option<String>,
    pub text_success: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionPreviewResponse {
    pub billing_country: Box<crate::models::CountryCode>,
    pub currency: Box<crate::models::Currency>,
    pub current_breakup: serde_json::Value,
    pub is_byop: bool,
    pub product_cart: Vec<serde_json::Value>,
    pub total_price: i64,
    pub recurring_breakup: Option<serde_json::Value>,
    pub tax_id_business_name: Option<String>,
    pub tax_id_err_msg: Option<String>,
    pub tax_id_format_name: Option<String>,
    pub total_tax: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttachExistingCustomer {
    pub customer_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BillingAddress {
    pub country: Box<crate::models::CountryCode>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    pub zipcode: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateNewCustomer {
    pub email: String,
    pub name: String,
    pub create_new_customer: Option<bool>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldResponse {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerLimitedDetails {
    pub customer_id: String,
    pub email: String,
    pub name: String,
    pub metadata: Option<Box<crate::models::Metadata>>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerRequest {
    AttachExistingCustomer(Box<crate::models::AttachExistingCustomer>),
    NewCustomer(Box<crate::models::NewCustomer>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IntentStatus {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "requires_customer_action")]
    RequiresCustomerAction,
    #[serde(rename = "requires_merchant_action")]
    RequiresMerchantAction,
    #[serde(rename = "requires_payment_method")]
    RequiresPaymentMethod,
    #[serde(rename = "requires_confirmation")]
    RequiresConfirmation,
    #[serde(rename = "requires_capture")]
    RequiresCapture,
    #[serde(rename = "partially_captured")]
    PartiallyCaptured,
    #[serde(rename = "partially_captured_and_capturable")]
    PartiallyCapturedAndCapturable,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewCustomer {
    pub email: String,
    pub name: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OneTimeProductCartItem {
    pub product_id: String,
    pub quantity: i64,
    pub amount: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payment {
    pub billing: Box<crate::models::BillingAddress>,
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub digital_products_delivered: bool,
    pub disputes: Vec<crate::models::Dispute>,
    pub metadata: Box<crate::models::Metadata>,
    pub payment_id: String,
    pub payment_provider: String,
    pub refunds: Vec<crate::models::RefundListItem>,
    pub retry_attempt: i64,
    pub settlement_amount: i64,
    pub settlement_currency: Box<crate::models::Currency>,
    pub total_amount: i64,
    pub card_holder_name: Option<String>,
    pub card_issuing_country: Option<Box<crate::models::CountryCode>>,
    pub card_last_four: Option<String>,
    pub card_network: Option<String>,
    pub card_type: Option<String>,
    pub checkout_session_id: Option<String>,
    pub custom_field_responses: Option<Vec<crate::models::CustomFieldResponse>>,
    pub discount_id: Option<String>,
    pub discounts: Option<Vec<crate::models::DiscountDetail>>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub invoice_id: Option<String>,
    pub invoice_url: Option<String>,
    pub payment_link: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_type: Option<String>,
    pub product_cart: Option<Vec<serde_json::Value>>,
    pub refund_status: Option<Box<crate::models::PaymentRefundStatus>>,
    pub settlement_tax: Option<i64>,
    pub status: Option<Box<crate::models::IntentStatus>>,
    pub subscription_id: Option<String>,
    pub tax: Option<i64>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PaymentMethodTypes {
    #[serde(rename = "ach")]
    ACH,
    #[serde(rename = "affirm")]
    Affirm,
    #[serde(rename = "afterpay_clearpay")]
    AfterpayClearpay,
    #[serde(rename = "alfamart")]
    Alfamart,
    #[serde(rename = "ali_pay")]
    AliPay,
    #[serde(rename = "ali_pay_hk")]
    AliPayHk,
    #[serde(rename = "alma")]
    Alma,
    #[serde(rename = "amazon_pay")]
    AmazonPay,
    #[serde(rename = "apple_pay")]
    ApplePay,
    #[serde(rename = "atome")]
    Atome,
    #[serde(rename = "bacs")]
    Bacs,
    #[serde(rename = "bancontact_card")]
    BancontactCard,
    #[serde(rename = "becs")]
    Becs,
    #[serde(rename = "benefit")]
    Benefit,
    #[serde(rename = "bizum")]
    Bizum,
    #[serde(rename = "blik")]
    Blik,
    #[serde(rename = "boleto")]
    Boleto,
    #[serde(rename = "bca_bank_transfer")]
    BcaBankTransfer,
    #[serde(rename = "bni_va")]
    BniVa,
    #[serde(rename = "bri_va")]
    BriVa,
    #[serde(rename = "card_redirect")]
    CardRedirect,
    #[serde(rename = "cimb_va")]
    CimbVa,
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "crypto_currency")]
    CryptoCurrency,
    #[serde(rename = "cashapp")]
    Cashapp,
    #[serde(rename = "dana")]
    Dana,
    #[serde(rename = "danamon_va")]
    DanamonVa,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "duit_now")]
    DuitNow,
    #[serde(rename = "efecty")]
    Efecty,
    #[serde(rename = "eft")]
    Eft,
    #[serde(rename = "eps")]
    Eps,
    #[serde(rename = "fps")]
    Fps,
    #[serde(rename = "evoucher")]
    Evoucher,
    #[serde(rename = "giropay")]
    Giropay,
    #[serde(rename = "givex")]
    Givex,
    #[serde(rename = "google_pay")]
    GooglePay,
    #[serde(rename = "go_pay")]
    GoPay,
    #[serde(rename = "gcash")]
    Gcash,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "interac")]
    Interac,
    #[serde(rename = "indomaret")]
    Indomaret,
    #[serde(rename = "klarna")]
    Klarna,
    #[serde(rename = "kakao_pay")]
    KakaoPay,
    #[serde(rename = "local_bank_redirect")]
    LocalBankRedirect,
    #[serde(rename = "mandiri_va")]
    MandiriVa,
    #[serde(rename = "knet")]
    Knet,
    #[serde(rename = "mb_way")]
    MBWay,
    #[serde(rename = "mobile_pay")]
    MobilePay,
    #[serde(rename = "momo")]
    Momo,
    #[serde(rename = "momo_atm")]
    MomoAtm,
    #[serde(rename = "multibanco")]
    Multibanco,
    #[serde(rename = "online_banking_thailand")]
    OnlineBankingThailand,
    #[serde(rename = "online_banking_czech_republic")]
    OnlineBankingCzechRepublic,
    #[serde(rename = "online_banking_finland")]
    OnlineBankingFinland,
    #[serde(rename = "online_banking_fpx")]
    OnlineBankingFpx,
    #[serde(rename = "online_banking_poland")]
    OnlineBankingPoland,
    #[serde(rename = "online_banking_slovakia")]
    OnlineBankingSlovakia,
    #[serde(rename = "oxxo")]
    Oxxo,
    #[serde(rename = "pago_efectivo")]
    PagoEfectivo,
    #[serde(rename = "permata_bank_transfer")]
    PermataBankTransfer,
    #[serde(rename = "open_banking_uk")]
    OpenBankingUk,
    #[serde(rename = "pay_bright")]
    PayBright,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "paze")]
    Paze,
    #[serde(rename = "pix")]
    Pix,
    #[serde(rename = "pay_safe_card")]
    PaySafeCard,
    #[serde(rename = "przelewy24")]
    Przelewy24,
    #[serde(rename = "prompt_pay")]
    PromptPay,
    #[serde(rename = "pse")]
    Pse,
    #[serde(rename = "red_compra")]
    RedCompra,
    #[serde(rename = "red_pagos")]
    RedPagos,
    #[serde(rename = "samsung_pay")]
    SamsungPay,
    #[serde(rename = "sepa")]
    Sepa,
    #[serde(rename = "sepa_bank_transfer")]
    SepaBankTransfer,
    #[serde(rename = "sofort")]
    Sofort,
    #[serde(rename = "sunbit")]
    Sunbit,
    #[serde(rename = "swish")]
    Swish,
    #[serde(rename = "touch_n_go")]
    TouchNGo,
    #[serde(rename = "trustly")]
    Trustly,
    #[serde(rename = "twint")]
    Twint,
    #[serde(rename = "upi_collect")]
    UpiCollect,
    #[serde(rename = "upi_intent")]
    UpiIntent,
    #[serde(rename = "vipps")]
    Vipps,
    #[serde(rename = "viet_qr")]
    VietQr,
    #[serde(rename = "venmo")]
    Venmo,
    #[serde(rename = "walley")]
    Walley,
    #[serde(rename = "we_chat_pay")]
    WeChatPay,
    #[serde(rename = "seven_eleven")]
    SevenEleven,
    #[serde(rename = "lawson")]
    Lawson,
    #[serde(rename = "mini_stop")]
    MiniStop,
    #[serde(rename = "family_mart")]
    FamilyMart,
    #[serde(rename = "seicomart")]
    Seicomart,
    #[serde(rename = "pay_easy")]
    PayEasy,
    #[serde(rename = "local_bank_transfer")]
    LocalBankTransfer,
    #[serde(rename = "mifinity")]
    Mifinity,
    #[serde(rename = "open_banking_pis")]
    OpenBankingPis,
    #[serde(rename = "direct_carrier_billing")]
    DirectCarrierBilling,
    #[serde(rename = "instant_bank_transfer")]
    InstantBankTransfer,
    #[serde(rename = "billie")]
    Billie,
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "revolut_pay")]
    RevolutPay,
    #[serde(rename = "naver_pay")]
    NaverPay,
    #[serde(rename = "payco")]
    Payco,
    #[serde(rename = "satispay")]
    Satispay,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PaymentRefundStatus {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "full")]
    Full,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefundListItem {
    pub business_id: String,
    pub created_at: String,
    pub is_partial: bool,
    pub payment_id: String,
    pub refund_id: String,
    pub status: Box<crate::models::RefundStatus>,
    pub amount: Option<i64>,
    pub currency: Option<Box<crate::models::Currency>>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentCreateResponse {
    pub client_secret: String,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub metadata: Box<crate::models::Metadata>,
    pub payment_id: String,
    pub total_amount: i64,
    pub discount_id: Option<String>,
    pub discount_ids: Option<Vec<String>>,
    pub expires_on: Option<String>,
    pub payment_link: Option<String>,
    pub product_cart: Option<Vec<crate::models::OneTimeProductCartItem>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentListResponse {
    pub brand_id: String,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub digital_products_delivered: bool,
    pub has_license_key: bool,
    pub metadata: Box<crate::models::Metadata>,
    pub payment_id: String,
    pub payment_provider: String,
    pub total_amount: i64,
    pub card_last_four: Option<String>,
    pub card_network: Option<String>,
    pub dispute_status: Option<Box<crate::models::DisputeStatus>>,
    pub invoice_id: Option<String>,
    pub invoice_url: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_type: Option<String>,
    pub refund_status: Option<Box<crate::models::PaymentRefundStatus>>,
    pub status: Option<Box<crate::models::IntentStatus>>,
    pub subscription_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentRetrieveLineItemsResponse {
    pub currency: Box<crate::models::Currency>,
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddonCartResponseItem {
    pub addon_id: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttachAddon {
    pub addon_id: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CancellationFeedback {
    #[serde(rename = "too_expensive")]
    TooExpensive,
    #[serde(rename = "missing_features")]
    MissingFeatures,
    #[serde(rename = "switched_service")]
    SwitchedService,
    #[serde(rename = "unused")]
    Unused,
    #[serde(rename = "customer_service")]
    CustomerService,
    #[serde(rename = "low_quality")]
    LowQuality,
    #[serde(rename = "too_complex")]
    TooComplex,
    #[serde(rename = "other")]
    Other,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditEntitlementCartResponse {
    pub credit_entitlement_id: String,
    pub credit_entitlement_name: String,
    pub credits_amount: String,
    pub overage_balance: String,
    pub overage_behavior: Box<crate::models::CbbOverageBehavior>,
    pub overage_enabled: bool,
    pub product_id: String,
    pub remaining_balance: String,
    pub rollover_enabled: bool,
    pub unit: String,
    pub expires_after_days: Option<i64>,
    pub low_balance_threshold_percent: Option<i64>,
    pub max_rollover_count: Option<i64>,
    pub overage_limit: Option<String>,
    pub rollover_percentage: Option<i64>,
    pub rollover_timeframe_count: Option<i64>,
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeterCartResponseItem {
    pub currency: Box<crate::models::Currency>,
    pub free_threshold: i64,
    pub measurement_unit: String,
    pub meter_id: String,
    pub name: String,
    pub description: Option<String>,
    pub price_per_unit: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeterCreditEntitlementCartResponse {
    pub credit_entitlement_id: String,
    pub meter_id: String,
    pub meter_name: String,
    pub meter_units_per_credit: String,
    pub product_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnDemandSubscription {
    pub mandate_only: bool,
    pub adaptive_currency_fees_inclusive: Option<bool>,
    pub product_currency: Option<Box<crate::models::Currency>>,
    pub product_description: Option<String>,
    pub product_price: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledPlanChange {
    pub id: String,
    pub addons: Vec<serde_json::Value>,
    pub created_at: String,
    pub effective_at: String,
    pub product_id: String,
    pub quantity: i64,
    pub product_description: Option<String>,
    pub product_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subscription {
    pub addons: Vec<crate::models::AddonCartResponseItem>,
    pub billing: Box<crate::models::BillingAddress>,
    pub brand_id: String,
    pub cancel_at_next_billing_date: bool,
    pub created_at: String,
    pub credit_entitlement_cart: Vec<crate::models::CreditEntitlementCartResponse>,
    pub currency: Box<crate::models::Currency>,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub metadata: Box<crate::models::Metadata>,
    pub meter_credit_entitlement_cart: Vec<crate::models::MeterCreditEntitlementCartResponse>,
    pub meters: Vec<crate::models::MeterCartResponseItem>,
    pub next_billing_date: String,
    pub on_demand: bool,
    pub payment_frequency_count: i64,
    pub payment_frequency_interval: Box<crate::models::TimeInterval>,
    pub previous_billing_date: String,
    pub product_id: String,
    pub quantity: i64,
    pub recurring_pre_tax_amount: i64,
    pub status: Box<crate::models::SubscriptionStatus>,
    pub subscription_id: String,
    pub subscription_period_count: i64,
    pub subscription_period_interval: Box<crate::models::TimeInterval>,
    pub tax_inclusive: bool,
    pub trial_period_days: i64,
    pub cancellation_comment: Option<String>,
    pub cancellation_feedback: Option<Box<crate::models::CancellationFeedback>>,
    pub cancelled_at: Option<String>,
    pub custom_field_responses: Option<Vec<crate::models::CustomFieldResponse>>,
    pub customer_business_name: Option<String>,
    pub discount_cycles_remaining: Option<i64>,
    pub discount_id: Option<String>,
    pub discounts: Option<Vec<crate::models::DiscountDetail>>,
    pub expires_at: Option<String>,
    pub payment_method_id: Option<String>,
    pub scheduled_change: Option<Box<crate::models::ScheduledPlanChange>>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SubscriptionStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "on_hold")]
    OnHold,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TimeInterval {
    Day,
    Week,
    Month,
    Year,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateSubscriptionPlanReq {
    pub product_id: String,
    pub proration_billing_mode: String,
    pub quantity: i64,
    pub adaptive_currency_fees_inclusive: Option<bool>,
    pub addons: Option<Vec<crate::models::AttachAddon>>,
    pub discount_code: Option<String>,
    pub discount_codes: Option<Vec<String>>,
    pub effective_at: Option<String>,
    pub metadata: Option<Box<crate::models::Metadata>>,
    pub on_payment_failure: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionCreateResponse {
    pub addons: Vec<crate::models::AddonCartResponseItem>,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub metadata: Box<crate::models::Metadata>,
    pub payment_id: String,
    pub recurring_pre_tax_amount: i64,
    pub subscription_id: String,
    pub client_secret: Option<String>,
    pub discount_id: Option<String>,
    pub discount_ids: Option<Vec<String>>,
    pub expires_on: Option<String>,
    pub one_time_product_cart: Option<Vec<serde_json::Value>>,
    pub payment_link: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionListResponse {
    pub billing: Box<crate::models::BillingAddress>,
    pub cancel_at_next_billing_date: bool,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub discounts: Vec<serde_json::Value>,
    pub metadata: Box<crate::models::Metadata>,
    pub next_billing_date: String,
    pub on_demand: bool,
    pub payment_frequency_count: i64,
    pub payment_frequency_interval: Box<crate::models::TimeInterval>,
    pub previous_billing_date: String,
    pub product_id: String,
    pub quantity: i64,
    pub recurring_pre_tax_amount: i64,
    pub status: Box<crate::models::SubscriptionStatus>,
    pub subscription_id: String,
    pub subscription_period_count: i64,
    pub subscription_period_interval: Box<crate::models::TimeInterval>,
    pub tax_inclusive: bool,
    pub trial_period_days: i64,
    pub cancelled_at: Option<String>,
    pub customer_business_name: Option<String>,
    pub discount_cycles_remaining: Option<i64>,
    pub discount_id: Option<String>,
    pub payment_method_id: Option<String>,
    pub product_name: Option<String>,
    pub scheduled_change: Option<Box<crate::models::ScheduledPlanChange>>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionChargeResponse {
    pub payment_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionPreviewChangePlanResponse {
    pub immediate_charge: serde_json::Value,
    pub new_plan: Box<crate::models::Subscription>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionRetrieveCreditUsageResponse {
    pub items: Vec<serde_json::Value>,
    pub subscription_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionRetrieveUsageHistoryResponse {
    pub end_date: String,
    pub meters: Vec<serde_json::Value>,
    pub start_date: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionUpdatePaymentMethodResponse {
    pub client_secret: Option<String>,
    pub expires_on: Option<String>,
    pub payment_id: Option<String>,
    pub payment_link: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseActivateResponse {
    pub id: String,
    pub business_id: String,
    pub created_at: String,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub license_key_id: String,
    pub name: String,
    pub product: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseValidateResponse {
    pub valid: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseKey {
    pub id: String,
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub customer_id: String,
    pub instances_count: i64,
    pub key: String,
    pub product_id: String,
    pub source: String,
    pub status: Box<crate::models::LicenseKeyStatus>,
    pub activations_limit: Option<i64>,
    pub expires_at: Option<String>,
    pub payment_id: Option<String>,
    pub subscription_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LicenseKeyStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeyInstance {
    pub id: String,
    pub business_id: String,
    pub created_at: String,
    pub license_key_id: String,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    pub business_id: String,
    pub created_at: String,
    pub customer_id: String,
    pub email: String,
    pub name: String,
    pub metadata: Option<Box<crate::models::Metadata>>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerPortalSession {
    pub link: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerListCreditEntitlementsResponse {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerListEntitlementsResponse {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerRetrievePaymentMethodsResponse {
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerWallet {
    pub balance: i64,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub customer_id: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletListResponse {
    pub items: Vec<crate::models::CustomerWallet>,
    pub total_balance_usd: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerWalletTransaction {
    pub id: String,
    pub after_balance: i64,
    pub amount: i64,
    pub before_balance: i64,
    pub business_id: String,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub customer_id: String,
    pub event_type: String,
    pub is_credit: bool,
    pub reason: Option<String>,
    pub reference_object_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Refund {
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub is_partial: bool,
    pub metadata: Box<crate::models::Metadata>,
    pub payment_id: String,
    pub refund_id: String,
    pub status: Box<crate::models::RefundStatus>,
    pub amount: Option<i64>,
    pub currency: Option<Box<crate::models::Currency>>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RefundStatus {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "review")]
    Review,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dispute {
    pub amount: String,
    pub business_id: String,
    pub created_at: String,
    pub currency: String,
    pub dispute_id: String,
    pub dispute_stage: Box<crate::models::DisputeStage>,
    pub dispute_status: Box<crate::models::DisputeStatus>,
    pub payment_id: String,
    pub is_resolved_by_rdr: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DisputeStage {
    #[serde(rename = "pre_dispute")]
    PreDispute,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "pre_arbitration")]
    PreArbitration,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DisputeStatus {
    #[serde(rename = "dispute_opened")]
    DisputeOpened,
    #[serde(rename = "dispute_expired")]
    DisputeExpired,
    #[serde(rename = "dispute_accepted")]
    DisputeAccepted,
    #[serde(rename = "dispute_cancelled")]
    DisputeCancelled,
    #[serde(rename = "dispute_challenged")]
    DisputeChallenged,
    #[serde(rename = "dispute_won")]
    DisputeWon,
    #[serde(rename = "dispute_lost")]
    DisputeLost,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetDispute {
    pub amount: String,
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub currency: String,
    pub customer: Box<crate::models::CustomerLimitedDetails>,
    pub dispute_id: String,
    pub dispute_stage: Box<crate::models::DisputeStage>,
    pub dispute_status: Box<crate::models::DisputeStatus>,
    pub payment_id: String,
    pub payment_provider: String,
    pub is_resolved_by_rdr: Option<bool>,
    pub reason: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeListResponse {
    pub amount: String,
    pub business_id: String,
    pub created_at: String,
    pub currency: String,
    pub dispute_id: String,
    pub dispute_stage: Box<crate::models::DisputeStage>,
    pub dispute_status: Box<crate::models::DisputeStatus>,
    pub payment_id: String,
    pub payment_provider: String,
    pub is_resolved_by_rdr: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PayoutListResponse {
    pub amount: i64,
    pub business_id: String,
    pub chargebacks: i64,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub fee: i64,
    pub payment_method: String,
    pub payout_id: String,
    pub refunds: i64,
    pub status: String,
    pub tax: i64,
    pub updated_at: String,
    pub name: Option<String>,
    pub payout_document_url: Option<String>,
    pub remarks: Option<String>,
}

pub type BreakupRetrieveResponse = Vec<serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DetailListResponse {
    pub id: String,
    pub created_at: String,
    pub event_type: String,
    pub original_amount: i64,
    pub original_currency: String,
    pub payout_currency_amount: i64,
    pub usd_equivalent_amount: i64,
    pub description: Option<String>,
    pub reference_object_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddMeterToPrice {
    pub meter_id: String,
    pub credit_entitlement_id: Option<String>,
    pub description: Option<String>,
    pub free_threshold: Option<i64>,
    pub measurement_unit: Option<String>,
    pub meter_units_per_credit: Option<String>,
    pub name: Option<String>,
    pub price_per_unit: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttachCreditEntitlement {
    pub credit_entitlement_id: String,
    pub credits_amount: String,
    pub currency: Option<Box<crate::models::Currency>>,
    pub expires_after_days: Option<i64>,
    pub low_balance_threshold_percent: Option<i64>,
    pub max_rollover_count: Option<i64>,
    pub overage_behavior: Option<Box<crate::models::CbbOverageBehavior>>,
    pub overage_enabled: Option<bool>,
    pub overage_limit: Option<String>,
    pub price_per_unit: Option<String>,
    pub proration_behavior: Option<Box<crate::models::CbbProrationBehavior>>,
    pub rollover_enabled: Option<bool>,
    pub rollover_percentage: Option<i64>,
    pub rollover_timeframe_count: Option<i64>,
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
    pub trial_credits: Option<String>,
    pub trial_credits_expire_after_trial: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttachProductEntitlement {
    pub entitlement_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CbbProrationBehavior {
    #[serde(rename = "prorate")]
    Prorate,
    #[serde(rename = "no_prorate")]
    NoProrate,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditEntitlementMappingResponse {
    pub id: String,
    pub credit_entitlement_id: String,
    pub credit_entitlement_name: String,
    pub credit_entitlement_unit: String,
    pub credits_amount: String,
    pub overage_behavior: Box<crate::models::CbbOverageBehavior>,
    pub overage_enabled: bool,
    pub proration_behavior: Box<crate::models::CbbProrationBehavior>,
    pub rollover_enabled: bool,
    pub trial_credits_expire_after_trial: bool,
    pub currency: Option<Box<crate::models::Currency>>,
    pub expires_after_days: Option<i64>,
    pub low_balance_threshold_percent: Option<i64>,
    pub max_rollover_count: Option<i64>,
    pub overage_limit: Option<String>,
    pub price_per_unit: Option<String>,
    pub rollover_percentage: Option<i64>,
    pub rollover_timeframe_count: Option<i64>,
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
    pub trial_credits: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DigitalProductDelivery {
    pub files: Vec<crate::models::DigitalProductDeliveryFile>,
    pub external_url: Option<String>,
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DigitalProductDeliveryFile {
    pub download_url: String,
    pub expires_in: i64,
    pub file_id: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeyDuration {
    pub count: i64,
    pub interval: Box<crate::models::TimeInterval>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Price {
    Variant0(serde_json::Value),
    Variant1(serde_json::Value),
    Variant2(serde_json::Value),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub credit_entitlements: Vec<crate::models::CreditEntitlementMappingResponse>,
    pub entitlements: Vec<crate::models::ProductEntitlementSummary>,
    pub is_recurring: bool,
    pub license_key_enabled: bool,
    pub metadata: Box<crate::models::Metadata>,
    pub price: Box<crate::models::Price>,
    pub product_id: String,
    pub tax_category: Box<crate::models::TaxCategory>,
    pub updated_at: String,
    pub addons: Option<Vec<String>>,
    pub description: Option<String>,
    pub digital_product_delivery: Option<Box<crate::models::DigitalProductDelivery>>,
    pub image: Option<String>,
    pub license_key_activation_message: Option<String>,
    pub license_key_activations_limit: Option<i64>,
    pub license_key_duration: Option<Box<crate::models::LicenseKeyDuration>>,
    pub name: Option<String>,
    pub pricing_mode: Option<Box<crate::models::PricingMode>>,
    pub product_collection_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductEntitlementSummary {
    pub id: String,
    pub integration_config: Box<crate::models::IntegrationConfigResponse>,
    pub integration_type: Box<crate::models::EntitlementIntegrationType>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductListResponse {
    pub business_id: String,
    pub created_at: String,
    pub entitlements: Vec<crate::models::ProductEntitlementSummary>,
    pub is_recurring: bool,
    pub metadata: Box<crate::models::Metadata>,
    pub product_id: String,
    pub tax_category: Box<crate::models::TaxCategory>,
    pub updated_at: String,
    pub currency: Option<Box<crate::models::Currency>>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub price: Option<i64>,
    pub price_detail: Option<Box<crate::models::Price>>,
    pub pricing_mode: Option<Box<crate::models::PricingMode>>,
    pub tax_inclusive: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductUpdateFilesResponse {
    pub file_id: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageUpdateResponse {
    pub url: String,
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortLinkCreateResponse {
    pub full_url: String,
    pub short_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortLinkListResponse {
    pub created_at: String,
    pub full_url: String,
    pub product_id: String,
    pub short_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListLocalizedPricesResponse {
    pub items: Vec<crate::models::LocalizedPrice>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalizedPrice {
    pub id: String,
    pub amount: i64,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub mode: Box<crate::models::PricingMode>,
    pub product_id: String,
    pub updated_at: String,
    pub country_code: Option<Box<crate::models::CountryCode>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PricingMode {
    #[serde(rename = "by_currency")]
    ByCurrency,
    #[serde(rename = "by_country")]
    ByCountry,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CountryCode {
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AO")]
    Ao,
    AI,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IN")]
    In,
    ID,
    #[serde(rename = "IR")]
    Ir,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KP")]
    Kp,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "GS")]
    Gs,
    SS,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "SY")]
    Sy,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "AE")]
    Ae,
    GB,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Currency {
    #[serde(rename = "AED")]
    Aed,
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AMD")]
    Amd,
    #[serde(rename = "ANG")]
    Ang,
    #[serde(rename = "AOA")]
    Aoa,
    #[serde(rename = "ARS")]
    Ars,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "AWG")]
    Awg,
    #[serde(rename = "AZN")]
    Azn,
    #[serde(rename = "BAM")]
    Bam,
    #[serde(rename = "BBD")]
    Bbd,
    #[serde(rename = "BDT")]
    Bdt,
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BHD")]
    Bhd,
    #[serde(rename = "BIF")]
    Bif,
    #[serde(rename = "BMD")]
    Bmd,
    #[serde(rename = "BND")]
    Bnd,
    #[serde(rename = "BOB")]
    Bob,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "BSD")]
    Bsd,
    #[serde(rename = "BWP")]
    Bwp,
    #[serde(rename = "BYN")]
    Byn,
    #[serde(rename = "BZD")]
    Bzd,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "CNY")]
    Cny,
    #[serde(rename = "COP")]
    Cop,
    #[serde(rename = "CRC")]
    Crc,
    #[serde(rename = "CUP")]
    Cup,
    #[serde(rename = "CVE")]
    Cve,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "DJF")]
    Djf,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "DOP")]
    Dop,
    #[serde(rename = "DZD")]
    Dzd,
    #[serde(rename = "EGP")]
    Egp,
    #[serde(rename = "ETB")]
    Etb,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "FJD")]
    Fjd,
    #[serde(rename = "FKP")]
    Fkp,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "GEL")]
    Gel,
    #[serde(rename = "GHS")]
    Ghs,
    #[serde(rename = "GIP")]
    Gip,
    #[serde(rename = "GMD")]
    Gmd,
    #[serde(rename = "GNF")]
    Gnf,
    #[serde(rename = "GTQ")]
    Gtq,
    #[serde(rename = "GYD")]
    Gyd,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "HNL")]
    Hnl,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HTG")]
    Htg,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "IDR")]
    Idr,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "INR")]
    Inr,
    #[serde(rename = "IQD")]
    Iqd,
    #[serde(rename = "JMD")]
    Jmd,
    #[serde(rename = "JOD")]
    Jod,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "KES")]
    Kes,
    #[serde(rename = "KGS")]
    Kgs,
    #[serde(rename = "KHR")]
    Khr,
    #[serde(rename = "KMF")]
    Kmf,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "KWD")]
    Kwd,
    #[serde(rename = "KYD")]
    Kyd,
    #[serde(rename = "KZT")]
    Kzt,
    #[serde(rename = "LAK")]
    Lak,
    #[serde(rename = "LBP")]
    Lbp,
    #[serde(rename = "LKR")]
    Lkr,
    #[serde(rename = "LRD")]
    Lrd,
    #[serde(rename = "LSL")]
    Lsl,
    #[serde(rename = "LYD")]
    Lyd,
    #[serde(rename = "MAD")]
    Mad,
    #[serde(rename = "MDL")]
    Mdl,
    #[serde(rename = "MGA")]
    Mga,
    #[serde(rename = "MKD")]
    Mkd,
    #[serde(rename = "MMK")]
    Mmk,
    #[serde(rename = "MNT")]
    Mnt,
    #[serde(rename = "MOP")]
    Mop,
    #[serde(rename = "MRU")]
    Mru,
    #[serde(rename = "MUR")]
    Mur,
    #[serde(rename = "MVR")]
    Mvr,
    #[serde(rename = "MWK")]
    Mwk,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "MYR")]
    Myr,
    #[serde(rename = "MZN")]
    Mzn,
    #[serde(rename = "NAD")]
    Nad,
    #[serde(rename = "NGN")]
    Ngn,
    #[serde(rename = "NIO")]
    Nio,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "NPR")]
    Npr,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "OMR")]
    Omr,
    #[serde(rename = "PAB")]
    Pab,
    #[serde(rename = "PEN")]
    Pen,
    #[serde(rename = "PGK")]
    Pgk,
    #[serde(rename = "PHP")]
    Php,
    #[serde(rename = "PKR")]
    Pkr,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "PYG")]
    Pyg,
    #[serde(rename = "QAR")]
    Qar,
    #[serde(rename = "RON")]
    Ron,
    #[serde(rename = "RSD")]
    Rsd,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "RWF")]
    Rwf,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "SBD")]
    Sbd,
    #[serde(rename = "SCR")]
    Scr,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "SGD")]
    Sgd,
    #[serde(rename = "SHP")]
    Shp,
    #[serde(rename = "SLE")]
    Sle,
    #[serde(rename = "SLL")]
    Sll,
    #[serde(rename = "SOS")]
    Sos,
    #[serde(rename = "SRD")]
    Srd,
    #[serde(rename = "SSP")]
    Ssp,
    #[serde(rename = "STN")]
    Stn,
    #[serde(rename = "SVC")]
    Svc,
    #[serde(rename = "SZL")]
    Szl,
    #[serde(rename = "THB")]
    Thb,
    #[serde(rename = "TND")]
    Tnd,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "TRY")]
    Try,
    #[serde(rename = "TTD")]
    Ttd,
    #[serde(rename = "TWD")]
    Twd,
    #[serde(rename = "TZS")]
    Tzs,
    #[serde(rename = "UAH")]
    Uah,
    #[serde(rename = "UGX")]
    Ugx,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "UYU")]
    Uyu,
    #[serde(rename = "UZS")]
    Uzs,
    #[serde(rename = "VES")]
    Ves,
    #[serde(rename = "VND")]
    Vnd,
    #[serde(rename = "VUV")]
    Vuv,
    #[serde(rename = "WST")]
    Wst,
    #[serde(rename = "XAF")]
    Xaf,
    #[serde(rename = "XCD")]
    Xcd,
    #[serde(rename = "XOF")]
    Xof,
    #[serde(rename = "XPF")]
    Xpf,
    #[serde(rename = "YER")]
    Yer,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "ZMW")]
    Zmw,
    #[serde(other)]
    Unknown,
}

pub type Metadata = std::collections::HashMap<String, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TaxCategory {
    #[serde(rename = "digital_products")]
    DigitalProducts,
    #[serde(rename = "saas")]
    Saas,
    #[serde(rename = "e_book")]
    EBook,
    #[serde(rename = "edtech")]
    Edtech,
    #[serde(other)]
    Unknown,
}

pub type MiscListSupportedCountriesResponse = Vec<crate::models::CountryCode>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Discount {
    pub amount: i64,
    pub business_id: String,
    pub code: String,
    pub created_at: String,
    pub discount_id: String,
    pub metadata: Box<crate::models::Metadata>,
    pub preserve_on_plan_change: bool,
    pub restricted_to: Vec<String>,
    pub times_used: i64,
    pub r#type: Box<crate::models::DiscountType>,
    pub expires_at: Option<String>,
    pub name: Option<String>,
    pub subscription_cycles: Option<i64>,
    pub usage_limit: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscountDetail {
    pub amount: i64,
    pub business_id: String,
    pub code: String,
    pub created_at: String,
    pub discount_id: String,
    pub metadata: Box<crate::models::Metadata>,
    pub position: i64,
    pub preserve_on_plan_change: bool,
    pub restricted_to: Vec<String>,
    pub times_used: i64,
    pub r#type: Box<crate::models::DiscountType>,
    pub cycles_remaining: Option<i64>,
    pub expires_at: Option<String>,
    pub name: Option<String>,
    pub subscription_cycles: Option<i64>,
    pub usage_limit: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DiscountType {
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddonResponse {
    pub id: String,
    pub business_id: String,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub name: String,
    pub price: i64,
    pub tax_category: Box<crate::models::TaxCategory>,
    pub updated_at: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddonUpdateImagesResponse {
    pub image_id: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Brand {
    pub brand_id: String,
    pub business_id: String,
    pub enabled: bool,
    pub statement_descriptor: String,
    pub verification_enabled: bool,
    pub verification_status: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub reason_for_hold: Option<String>,
    pub support_email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrandListResponse {
    pub items: Vec<crate::models::Brand>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrandUpdateImagesResponse {
    pub image_id: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookDetails {
    pub id: String,
    pub created_at: String,
    pub description: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub updated_at: String,
    pub url: String,
    pub disabled: Option<bool>,
    pub filter_types: Option<Vec<String>>,
    pub rate_limit: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookRetrieveSecretResponse {
    pub secret: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AbandonedCheckoutDetectedWebhookEvent {
    pub business_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AbandonedCheckoutRecoveredWebhookEvent {
    pub business_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditAddedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditBalanceLowWebhookEvent {
    pub business_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditDeductedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditExpiredWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditManualAdjustmentWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditOverageChargedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditOverageResetWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditRolledOverWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditRolloverForfeitedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::CreditLedgerEntry>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeAcceptedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeCancelledWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeChallengedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeExpiredWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeLostWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeOpenedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisputeWonWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Dispute>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DunningRecoveredWebhookEvent {
    pub business_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DunningStartedWebhookEvent {
    pub business_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntitlementGrantCreatedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::EntitlementGrant>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntitlementGrantDeliveredWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::EntitlementGrant>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntitlementGrantFailedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::EntitlementGrant>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntitlementGrantRevokedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::EntitlementGrant>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeyCreatedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::LicenseKey>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentCancelledWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Payment>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentFailedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Payment>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentProcessingWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Payment>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentSucceededWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Payment>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefundFailedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Refund>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefundSucceededWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Refund>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionActiveWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionCancelledWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionExpiredWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionFailedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionOnHoldWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionPlanChangedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionRenewedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionUpdatedWebhookEvent {
    pub business_id: String,
    pub data: Box<crate::models::Subscription>,
    pub timestamp: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum UnsafeUnwrapWebhookEvent {
    AbandonedCheckoutDetectedWebhookEvent(
        Box<crate::models::AbandonedCheckoutDetectedWebhookEvent>,
    ),
    AbandonedCheckoutRecoveredWebhookEvent(
        Box<crate::models::AbandonedCheckoutRecoveredWebhookEvent>,
    ),
    CreditAddedWebhookEvent(Box<crate::models::CreditAddedWebhookEvent>),
    CreditBalanceLowWebhookEvent(Box<crate::models::CreditBalanceLowWebhookEvent>),
    CreditDeductedWebhookEvent(Box<crate::models::CreditDeductedWebhookEvent>),
    CreditExpiredWebhookEvent(Box<crate::models::CreditExpiredWebhookEvent>),
    CreditManualAdjustmentWebhookEvent(Box<crate::models::CreditManualAdjustmentWebhookEvent>),
    CreditOverageChargedWebhookEvent(Box<crate::models::CreditOverageChargedWebhookEvent>),
    CreditOverageResetWebhookEvent(Box<crate::models::CreditOverageResetWebhookEvent>),
    CreditRolledOverWebhookEvent(Box<crate::models::CreditRolledOverWebhookEvent>),
    CreditRolloverForfeitedWebhookEvent(Box<crate::models::CreditRolloverForfeitedWebhookEvent>),
    DisputeAcceptedWebhookEvent(Box<crate::models::DisputeAcceptedWebhookEvent>),
    DisputeCancelledWebhookEvent(Box<crate::models::DisputeCancelledWebhookEvent>),
    DisputeChallengedWebhookEvent(Box<crate::models::DisputeChallengedWebhookEvent>),
    DisputeExpiredWebhookEvent(Box<crate::models::DisputeExpiredWebhookEvent>),
    DisputeLostWebhookEvent(Box<crate::models::DisputeLostWebhookEvent>),
    DisputeOpenedWebhookEvent(Box<crate::models::DisputeOpenedWebhookEvent>),
    DisputeWonWebhookEvent(Box<crate::models::DisputeWonWebhookEvent>),
    DunningRecoveredWebhookEvent(Box<crate::models::DunningRecoveredWebhookEvent>),
    DunningStartedWebhookEvent(Box<crate::models::DunningStartedWebhookEvent>),
    EntitlementGrantCreatedWebhookEvent(Box<crate::models::EntitlementGrantCreatedWebhookEvent>),
    EntitlementGrantDeliveredWebhookEvent(
        Box<crate::models::EntitlementGrantDeliveredWebhookEvent>,
    ),
    EntitlementGrantFailedWebhookEvent(Box<crate::models::EntitlementGrantFailedWebhookEvent>),
    EntitlementGrantRevokedWebhookEvent(Box<crate::models::EntitlementGrantRevokedWebhookEvent>),
    LicenseKeyCreatedWebhookEvent(Box<crate::models::LicenseKeyCreatedWebhookEvent>),
    PaymentCancelledWebhookEvent(Box<crate::models::PaymentCancelledWebhookEvent>),
    PaymentFailedWebhookEvent(Box<crate::models::PaymentFailedWebhookEvent>),
    PaymentProcessingWebhookEvent(Box<crate::models::PaymentProcessingWebhookEvent>),
    PaymentSucceededWebhookEvent(Box<crate::models::PaymentSucceededWebhookEvent>),
    RefundFailedWebhookEvent(Box<crate::models::RefundFailedWebhookEvent>),
    RefundSucceededWebhookEvent(Box<crate::models::RefundSucceededWebhookEvent>),
    SubscriptionActiveWebhookEvent(Box<crate::models::SubscriptionActiveWebhookEvent>),
    SubscriptionCancelledWebhookEvent(Box<crate::models::SubscriptionCancelledWebhookEvent>),
    SubscriptionExpiredWebhookEvent(Box<crate::models::SubscriptionExpiredWebhookEvent>),
    SubscriptionFailedWebhookEvent(Box<crate::models::SubscriptionFailedWebhookEvent>),
    SubscriptionOnHoldWebhookEvent(Box<crate::models::SubscriptionOnHoldWebhookEvent>),
    SubscriptionPlanChangedWebhookEvent(Box<crate::models::SubscriptionPlanChangedWebhookEvent>),
    SubscriptionRenewedWebhookEvent(Box<crate::models::SubscriptionRenewedWebhookEvent>),
    SubscriptionUpdatedWebhookEvent(Box<crate::models::SubscriptionUpdatedWebhookEvent>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum UnwrapWebhookEvent {
    AbandonedCheckoutDetectedWebhookEvent(
        Box<crate::models::AbandonedCheckoutDetectedWebhookEvent>,
    ),
    AbandonedCheckoutRecoveredWebhookEvent(
        Box<crate::models::AbandonedCheckoutRecoveredWebhookEvent>,
    ),
    CreditAddedWebhookEvent(Box<crate::models::CreditAddedWebhookEvent>),
    CreditBalanceLowWebhookEvent(Box<crate::models::CreditBalanceLowWebhookEvent>),
    CreditDeductedWebhookEvent(Box<crate::models::CreditDeductedWebhookEvent>),
    CreditExpiredWebhookEvent(Box<crate::models::CreditExpiredWebhookEvent>),
    CreditManualAdjustmentWebhookEvent(Box<crate::models::CreditManualAdjustmentWebhookEvent>),
    CreditOverageChargedWebhookEvent(Box<crate::models::CreditOverageChargedWebhookEvent>),
    CreditOverageResetWebhookEvent(Box<crate::models::CreditOverageResetWebhookEvent>),
    CreditRolledOverWebhookEvent(Box<crate::models::CreditRolledOverWebhookEvent>),
    CreditRolloverForfeitedWebhookEvent(Box<crate::models::CreditRolloverForfeitedWebhookEvent>),
    DisputeAcceptedWebhookEvent(Box<crate::models::DisputeAcceptedWebhookEvent>),
    DisputeCancelledWebhookEvent(Box<crate::models::DisputeCancelledWebhookEvent>),
    DisputeChallengedWebhookEvent(Box<crate::models::DisputeChallengedWebhookEvent>),
    DisputeExpiredWebhookEvent(Box<crate::models::DisputeExpiredWebhookEvent>),
    DisputeLostWebhookEvent(Box<crate::models::DisputeLostWebhookEvent>),
    DisputeOpenedWebhookEvent(Box<crate::models::DisputeOpenedWebhookEvent>),
    DisputeWonWebhookEvent(Box<crate::models::DisputeWonWebhookEvent>),
    DunningRecoveredWebhookEvent(Box<crate::models::DunningRecoveredWebhookEvent>),
    DunningStartedWebhookEvent(Box<crate::models::DunningStartedWebhookEvent>),
    EntitlementGrantCreatedWebhookEvent(Box<crate::models::EntitlementGrantCreatedWebhookEvent>),
    EntitlementGrantDeliveredWebhookEvent(
        Box<crate::models::EntitlementGrantDeliveredWebhookEvent>,
    ),
    EntitlementGrantFailedWebhookEvent(Box<crate::models::EntitlementGrantFailedWebhookEvent>),
    EntitlementGrantRevokedWebhookEvent(Box<crate::models::EntitlementGrantRevokedWebhookEvent>),
    LicenseKeyCreatedWebhookEvent(Box<crate::models::LicenseKeyCreatedWebhookEvent>),
    PaymentCancelledWebhookEvent(Box<crate::models::PaymentCancelledWebhookEvent>),
    PaymentFailedWebhookEvent(Box<crate::models::PaymentFailedWebhookEvent>),
    PaymentProcessingWebhookEvent(Box<crate::models::PaymentProcessingWebhookEvent>),
    PaymentSucceededWebhookEvent(Box<crate::models::PaymentSucceededWebhookEvent>),
    RefundFailedWebhookEvent(Box<crate::models::RefundFailedWebhookEvent>),
    RefundSucceededWebhookEvent(Box<crate::models::RefundSucceededWebhookEvent>),
    SubscriptionActiveWebhookEvent(Box<crate::models::SubscriptionActiveWebhookEvent>),
    SubscriptionCancelledWebhookEvent(Box<crate::models::SubscriptionCancelledWebhookEvent>),
    SubscriptionExpiredWebhookEvent(Box<crate::models::SubscriptionExpiredWebhookEvent>),
    SubscriptionFailedWebhookEvent(Box<crate::models::SubscriptionFailedWebhookEvent>),
    SubscriptionOnHoldWebhookEvent(Box<crate::models::SubscriptionOnHoldWebhookEvent>),
    SubscriptionPlanChangedWebhookEvent(Box<crate::models::SubscriptionPlanChangedWebhookEvent>),
    SubscriptionRenewedWebhookEvent(Box<crate::models::SubscriptionRenewedWebhookEvent>),
    SubscriptionUpdatedWebhookEvent(Box<crate::models::SubscriptionUpdatedWebhookEvent>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeaderRetrieveResponse {
    pub headers: std::collections::HashMap<String, String>,
    pub sensitive: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "payment.succeeded")]
    PaymentSucceeded,
    #[serde(rename = "payment.failed")]
    PaymentFailed,
    #[serde(rename = "payment.processing")]
    PaymentProcessing,
    #[serde(rename = "payment.cancelled")]
    PaymentCancelled,
    #[serde(rename = "refund.succeeded")]
    RefundSucceeded,
    #[serde(rename = "refund.failed")]
    RefundFailed,
    #[serde(rename = "dispute.opened")]
    DisputeOpened,
    #[serde(rename = "dispute.expired")]
    DisputeExpired,
    #[serde(rename = "dispute.accepted")]
    DisputeAccepted,
    #[serde(rename = "dispute.cancelled")]
    DisputeCancelled,
    #[serde(rename = "dispute.challenged")]
    DisputeChallenged,
    #[serde(rename = "dispute.won")]
    DisputeWon,
    #[serde(rename = "dispute.lost")]
    DisputeLost,
    #[serde(rename = "subscription.active")]
    SubscriptionActive,
    #[serde(rename = "subscription.renewed")]
    SubscriptionRenewed,
    #[serde(rename = "subscription.on_hold")]
    SubscriptionOnHold,
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused,
    #[serde(rename = "subscription.cancelled")]
    SubscriptionCancelled,
    #[serde(rename = "subscription.failed")]
    SubscriptionFailed,
    #[serde(rename = "subscription.expired")]
    SubscriptionExpired,
    #[serde(rename = "subscription.plan_changed")]
    SubscriptionPlanChanged,
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,
    #[serde(rename = "license_key.created")]
    LicenseKeyCreated,
    #[serde(rename = "payout.not_initiated")]
    PayoutNotInitiated,
    #[serde(rename = "payout.on_hold")]
    PayoutOnHold,
    #[serde(rename = "payout.in_progress")]
    PayoutInProgress,
    #[serde(rename = "payout.failed")]
    PayoutFailed,
    #[serde(rename = "payout.success")]
    PayoutSuccess,
    #[serde(rename = "credit.added")]
    CreditAdded,
    #[serde(rename = "credit.deducted")]
    CreditDeducted,
    #[serde(rename = "credit.expired")]
    CreditExpired,
    #[serde(rename = "credit.rolled_over")]
    CreditRolledOver,
    #[serde(rename = "credit.rollover_forfeited")]
    CreditRolloverForfeited,
    #[serde(rename = "credit.overage_charged")]
    CreditOverageCharged,
    #[serde(rename = "credit.overage_reset")]
    CreditOverageReset,
    #[serde(rename = "credit.manual_adjustment")]
    CreditManualAdjustment,
    #[serde(rename = "credit.balance_low")]
    CreditBalanceLow,
    #[serde(rename = "abandoned_checkout.detected")]
    AbandonedCheckoutDetected,
    #[serde(rename = "abandoned_checkout.recovered")]
    AbandonedCheckoutRecovered,
    #[serde(rename = "dunning.started")]
    DunningStarted,
    #[serde(rename = "dunning.recovered")]
    DunningRecovered,
    #[serde(rename = "entitlement_grant.created")]
    EntitlementGrantCreated,
    #[serde(rename = "entitlement_grant.delivered")]
    EntitlementGrantDelivered,
    #[serde(rename = "entitlement_grant.failed")]
    EntitlementGrantFailed,
    #[serde(rename = "entitlement_grant.revoked")]
    EntitlementGrantRevoked,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookPayload {
    pub business_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub r#type: Box<crate::models::WebhookEventType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub business_id: String,
    pub customer_id: String,
    pub event_id: String,
    pub event_name: String,
    pub timestamp: String,
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventInput {
    pub customer_id: String,
    pub event_id: String,
    pub event_name: String,
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsageEventIngestResponse {
    pub ingested_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Conjunction {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FilterOperator {
    #[serde(rename = "equals")]
    Equals,
    #[serde(rename = "not_equals")]
    NotEquals,
    #[serde(rename = "greater_than")]
    GreaterThan,
    #[serde(rename = "greater_than_or_equals")]
    GreaterThanOrEquals,
    #[serde(rename = "less_than")]
    LessThan,
    #[serde(rename = "less_than_or_equals")]
    LessThanOrEquals,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "does_not_contain")]
    DoesNotContain,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum FilterType {
    List(Vec<serde_json::Value>),
    List2(Vec<crate::models::MeterFilter>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Meter {
    pub id: String,
    pub aggregation: Box<crate::models::MeterAggregation>,
    pub business_id: String,
    pub created_at: String,
    pub event_name: String,
    pub measurement_unit: String,
    pub name: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub filter: Option<Box<crate::models::MeterFilter>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeterAggregation {
    pub r#type: String,
    pub key: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeterFilter {
    pub clauses: Box<crate::models::FilterType>,
    pub conjunction: Box<crate::models::Conjunction>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BalanceLedgerEntry {
    pub id: String,
    pub amount: i64,
    pub business_id: String,
    pub created_at: String,
    pub currency: Box<crate::models::Currency>,
    pub event_type: String,
    pub is_credit: bool,
    pub usd_equivalent_amount: i64,
    pub after_balance: Option<i64>,
    pub before_balance: Option<i64>,
    pub description: Option<String>,
    pub reference_object_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CbbOverageBehavior {
    #[serde(rename = "forgive_at_reset")]
    ForgiveAtReset,
    #[serde(rename = "invoice_at_billing")]
    InvoiceAtBilling,
    #[serde(rename = "carry_deficit")]
    CarryDeficit,
    #[serde(rename = "carry_deficit_auto_repay")]
    CarryDeficitAutoRepay,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditEntitlement {
    pub id: String,
    pub business_id: String,
    pub created_at: String,
    pub name: String,
    pub overage_behavior: Box<crate::models::CbbOverageBehavior>,
    pub overage_enabled: bool,
    pub precision: i64,
    pub rollover_enabled: bool,
    pub unit: String,
    pub updated_at: String,
    pub currency: Option<Box<crate::models::Currency>>,
    pub description: Option<String>,
    pub expires_after_days: Option<i64>,
    pub max_rollover_count: Option<i64>,
    pub overage_limit: Option<i64>,
    pub price_per_unit: Option<String>,
    pub rollover_percentage: Option<i64>,
    pub rollover_timeframe_count: Option<i64>,
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreditLedgerEntry {
    pub id: String,
    pub amount: String,
    pub balance_after: String,
    pub balance_before: String,
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub credit_entitlement_id: String,
    pub customer_id: String,
    pub is_credit: bool,
    pub metadata: Box<crate::models::Metadata>,
    pub overage_after: String,
    pub overage_before: String,
    pub transaction_type: String,
    pub description: Option<String>,
    pub grant_id: Option<String>,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerCreditBalance {
    pub id: String,
    pub balance: String,
    pub created_at: String,
    pub credit_entitlement_id: String,
    pub customer_id: String,
    pub overage: String,
    pub updated_at: String,
    pub last_transaction_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LedgerEntryType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BalanceCreateLedgerEntryResponse {
    pub id: String,
    pub amount: String,
    pub balance_after: String,
    pub balance_before: String,
    pub created_at: String,
    pub credit_entitlement_id: String,
    pub customer_id: String,
    pub entry_type: Box<crate::models::LedgerEntryType>,
    pub is_credit: bool,
    pub overage_after: String,
    pub overage_before: String,
    pub grant_id: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BalanceListGrantsResponse {
    pub id: String,
    pub created_at: String,
    pub credit_entitlement_id: String,
    pub customer_id: String,
    pub initial_amount: String,
    pub is_expired: bool,
    pub is_rolled_over: bool,
    pub remaining_amount: String,
    pub rollover_count: i64,
    pub source_type: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
    pub metadata: Option<Box<crate::models::Metadata>>,
    pub parent_grant_id: Option<String>,
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Entitlement {
    pub id: String,
    pub business_id: String,
    pub created_at: String,
    pub integration_config: Box<crate::models::IntegrationConfigResponse>,
    pub integration_type: Box<crate::models::EntitlementIntegrationType>,
    pub is_active: bool,
    pub metadata: Box<crate::models::Metadata>,
    pub name: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EntitlementIntegrationType {
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "figma")]
    Figma,
    #[serde(rename = "framer")]
    Framer,
    #[serde(rename = "notion")]
    Notion,
    #[serde(rename = "digital_files")]
    DigitalFiles,
    #[serde(rename = "license_key")]
    LicenseKey,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum GitHubPermission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "triage")]
    Triage,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum IntegrationConfig {
    Variant0(serde_json::Value),
    Variant1(serde_json::Value),
    Variant2(serde_json::Value),
    Variant3(serde_json::Value),
    Variant4(serde_json::Value),
    Variant5(serde_json::Value),
    Variant6(serde_json::Value),
    Variant7(serde_json::Value),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum IntegrationConfigResponse {
    Variant0(serde_json::Value),
    Variant1(serde_json::Value),
    Variant2(serde_json::Value),
    Variant3(serde_json::Value),
    Variant4(serde_json::Value),
    Variant5(serde_json::Value),
    Variant6(serde_json::Value),
    Variant7(serde_json::Value),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileUploadResponse {
    pub file_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntitlementGrant {
    pub id: String,
    pub brand_id: String,
    pub business_id: String,
    pub created_at: String,
    pub customer_id: String,
    pub entitlement_id: String,
    pub integration_type: Box<crate::models::EntitlementIntegrationType>,
    pub metadata: Box<crate::models::Metadata>,
    pub status: String,
    pub updated_at: String,
    pub delivered_at: Option<String>,
    pub digital_product_delivery: Option<Box<crate::models::DigitalProductDelivery>>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub license_key: Option<Box<crate::models::LicenseKeyGrant>>,
    pub oauth_expires_at: Option<String>,
    pub oauth_url: Option<String>,
    pub payment_id: Option<String>,
    pub revocation_reason: Option<String>,
    pub revoked_at: Option<String>,
    pub subscription_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeyGrant {
    pub activations_used: i64,
    pub key: String,
    pub activations_limit: Option<i64>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollection {
    pub id: String,
    pub brand_id: String,
    pub created_at: String,
    pub groups: Vec<crate::models::ProductCollectionGroupResponse>,
    pub name: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub effective_at_on_downgrade: Option<String>,
    pub effective_at_on_upgrade: Option<String>,
    pub image: Option<String>,
    pub on_payment_failure: Option<String>,
    pub proration_billing_mode_on_downgrade: Option<String>,
    pub proration_billing_mode_on_upgrade: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionListResponse {
    pub id: String,
    pub created_at: String,
    pub name: String,
    pub products_count: i64,
    pub updated_at: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionUnarchiveResponse {
    pub collection_id: String,
    pub excluded_product_ids: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionUpdateImagesResponse {
    pub url: String,
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupProduct {
    pub product_id: String,
    pub status: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionGroupDetails {
    pub products: Vec<crate::models::GroupProduct>,
    pub group_name: Option<String>,
    pub status: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionGroupResponse {
    pub group_id: String,
    pub products: Vec<crate::models::ProductCollectionProduct>,
    pub status: bool,
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionProduct {
    pub id: String,
    pub addons_count: i64,
    pub files_count: i64,
    pub has_credit_entitlements: bool,
    pub is_recurring: bool,
    pub license_key_enabled: bool,
    pub meters_count: i64,
    pub product_id: String,
    pub status: bool,
    pub currency: Option<Box<crate::models::Currency>>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub price: Option<i64>,
    pub price_detail: Option<Box<crate::models::Price>>,
    pub tax_category: Option<Box<crate::models::TaxCategory>>,
    pub tax_inclusive: Option<bool>,
}

pub type ItemCreateResponse = Vec<crate::models::ProductCollectionProduct>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DefaultPageNumberPagination<T> {
    pub items: Vec<T>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
    #[serde(skip)]
    pagination_context: Option<crate::client::PaginationContext>,
}

impl<T> DefaultPageNumberPagination<T> {
    pub(crate) fn set_pagination_context(&mut self, context: crate::client::PaginationContext) {
        self.pagination_context = Some(context);
    }
}

impl<T: serde::de::DeserializeOwned> DefaultPageNumberPagination<T> {
    pub async fn get_next_page(&self) -> crate::error::Result<Option<Self>> {
        if self.items.is_empty() {
            return Ok(None);
        }
        let Some(context) = self.pagination_context.as_ref() else {
            return Ok(None);
        };
        let current = context.int_param("page_number");
        let next_context = context.with_param("page_number", serde_json::Value::from(current + 1));
        let mut next: Self = next_context.fetch().await?;
        next.pagination_context = Some(next_context);
        Ok(Some(next))
    }
}

impl<T: serde::de::DeserializeOwned + Clone> DefaultPageNumberPagination<T> {
    pub fn into_stream(self) -> impl futures::Stream<Item = crate::error::Result<T>> {
        futures::stream::unfold(Some((self, 0usize)), |state| async move {
            let (page, index) = state?;
            if index < page.items.len() {
                let item = page.items[index].clone();
                return Some((Ok(item), Some((page, index + 1))));
            }
            match page.get_next_page().await {
                Ok(Some(next)) if !next.items.is_empty() => {
                    let item = next.items[0].clone();
                    Some((Ok(item), Some((next, 1))))
                }
                Ok(_) => None,
                Err(error) => Some((Err(error), None)),
            }
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CursorPagePagination<T> {
    pub data: Vec<T>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
    #[serde(skip)]
    pagination_context: Option<crate::client::PaginationContext>,
}

impl<T> CursorPagePagination<T> {
    pub(crate) fn set_pagination_context(&mut self, context: crate::client::PaginationContext) {
        self.pagination_context = Some(context);
    }
}

impl<T: serde::de::DeserializeOwned> CursorPagePagination<T> {
    pub async fn get_next_page(&self) -> crate::error::Result<Option<Self>> {
        if self.data.is_empty() {
            return Ok(None);
        }
        let Some(cursor) = self
            .extra
            .get("iterator")
            .and_then(|value| value.as_str())
            .filter(|value| !value.is_empty())
        else {
            return Ok(None);
        };
        let Some(context) = self.pagination_context.as_ref() else {
            return Ok(None);
        };
        let next_context = context.with_param("iterator", serde_json::Value::from(cursor));
        let mut next: Self = next_context.fetch().await?;
        next.pagination_context = Some(next_context);
        Ok(Some(next))
    }
}

impl<T: serde::de::DeserializeOwned + Clone> CursorPagePagination<T> {
    pub fn into_stream(self) -> impl futures::Stream<Item = crate::error::Result<T>> {
        futures::stream::unfold(Some((self, 0usize)), |state| async move {
            let (page, index) = state?;
            if index < page.data.len() {
                let item = page.data[index].clone();
                return Some((Ok(item), Some((page, index + 1))));
            }
            match page.get_next_page().await {
                Ok(Some(next)) if !next.data.is_empty() => {
                    let item = next.data[0].clone();
                    Some((Ok(item), Some((next, 1))))
                }
                Ok(_) => None,
                Err(error) => Some((Err(error), None)),
            }
        })
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSessionsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_cart: Option<Vec<crate::models::ProductItemReq>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_payment_method_types: Option<Vec<crate::models::PaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Box<crate::models::CheckoutSessionBillingAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<crate::models::CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<crate::models::CustomerRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization: Option<Box<crate::models::CheckoutSessionCustomization>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Box<crate::models::CheckoutSessionFlags>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_3ds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_min_amount_inr_paise: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_saved_payment_methods: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<Box<crate::models::SubscriptionData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<Box<crate::models::BillingAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<crate::models::CustomerRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_cart: Option<Vec<crate::models::OneTimeProductCartItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_currency_fees_inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_payment_method_types: Option<Vec<crate::models::PaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_3ds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_immediately: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_saved_payment_methods: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<Box<crate::models::BillingAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<crate::models::CustomerRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<crate::models::AttachAddon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_payment_method_types: Option<Vec<crate::models::PaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_3ds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_min_amount_inr_paise: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<Box<crate::models::OnDemandSubscription>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_product_cart: Option<Vec<crate::models::OneTimeProductCartItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_immediately: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_saved_payment_methods: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsUpdateParamsCreditEntitlementCartItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_entitlement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_balance_threshold_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rollover_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_percentage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_timeframe_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsUpdateParamsDisableOnDemand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<Box<crate::models::BillingAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_next_billing_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_feedback: Option<Box<crate::models::CancellationFeedback>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_entitlement_cart:
        Option<Vec<crate::models::SubscriptionsUpdateParamsCreditEntitlementCartItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_on_demand: Option<Box<crate::models::SubscriptionsUpdateParamsDisableOnDemand>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::SubscriptionStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsChangePlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_billing_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_currency_fees_inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<crate::models::AttachAddon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_payment_failure: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsChargeParamsCustomerBalanceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_customer_credits_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_customer_credits_usage: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsChargeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_currency_fees_inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance_config:
        Option<Box<crate::models::SubscriptionsChargeParamsCustomerBalanceConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicensesActivateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicensesDeactivateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicensesValidateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeysCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activations_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeysUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activations_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicenseKeyInstancesUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomersCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomersUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomersWalletsLedgerEntriesCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RefundsCreateParamsItemsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RefundsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::RefundsCreateParamsItemsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsCreateParamsDigitalProductDelivery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::models::Price>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<Box<crate::models::TaxCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_entitlements: Option<Vec<crate::models::AttachCreditEntitlement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_product_delivery:
        Option<Box<crate::models::ProductsCreateParamsDigitalProductDelivery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<crate::models::AttachProductEntitlement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_activation_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_activations_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_duration: Option<Box<crate::models::LicenseKeyDuration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_mode: Option<Box<crate::models::PricingMode>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsUpdateParamsDigitalProductDelivery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_entitlements: Option<Vec<crate::models::AttachCreditEntitlement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_product_delivery:
        Option<Box<crate::models::ProductsUpdateParamsDigitalProductDelivery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<crate::models::AttachProductEntitlement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_activation_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_activations_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_duration: Option<Box<crate::models::LicenseKeyDuration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::models::Price>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_mode: Option<Box<crate::models::PricingMode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<Box<crate::models::TaxCategory>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsUpdateFilesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsShortLinksCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_checkout_params: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsLocalizedPricesCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Box<crate::models::CountryCode>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductsLocalizedPricesUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DiscountsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::DiscountType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_on_plan_change: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cycles: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DiscountsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_on_plan_change: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cycles: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::DiscountType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddonsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<Box<crate::models::TaxCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddonsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<Box<crate::models::TaxCategory>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BrandsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BrandsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WebhooksCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<crate::models::WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WebhooksUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<crate::models::WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WebhooksHeadersUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UsageEventsIngestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::EventInput>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MetersCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<Box<crate::models::MeterAggregation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::MeterFilter>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreditEntitlementsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rollover_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_behavior: Option<Box<crate::models::CbbOverageBehavior>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_percentage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_timeframe_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreditEntitlementsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rollover_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_behavior: Option<Box<crate::models::CbbOverageBehavior>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_percentage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_timeframe_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_timeframe_interval: Option<Box<crate::models::TimeInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CreditEntitlementsBalancesCreateLedgerEntryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<Box<crate::models::LedgerEntryType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EntitlementsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<Box<crate::models::IntegrationConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<Box<crate::models::EntitlementIntegrationType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EntitlementsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_config: Option<Box<crate::models::IntegrationConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EntitlementsGrantsFulfillLicenseKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activations_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::ProductCollectionGroupDetails>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at_on_downgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at_on_upgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_payment_failure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_billing_mode_on_downgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_billing_mode_on_upgrade: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at_on_downgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at_on_upgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_payment_failure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_billing_mode_on_downgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_billing_mode_on_upgrade: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionsGroupsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<crate::models::GroupProduct>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionsGroupsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionsGroupsItemsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<crate::models::GroupProduct>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductCollectionsGroupsItemsUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}
