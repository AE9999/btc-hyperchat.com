export interface IStoreWebhookConfig {
  url: string | undefined,
  postTypeCode: number | undefined,
  dataAttributes: Record<string, string> | undefined,
  queryAttributes: Record<string, string> | undefined,
  headerAttributes: Record<string, string> | undefined,
}
