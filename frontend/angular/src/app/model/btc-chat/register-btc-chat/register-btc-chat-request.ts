export interface IRegisterBtcChatRequest {
  storeId: string,
  currency: string,
  amount: number,
  message: string | undefined,
  sender: string | undefined,
}
