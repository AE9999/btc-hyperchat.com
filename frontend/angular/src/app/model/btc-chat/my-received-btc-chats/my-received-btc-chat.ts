export interface IMyReceivedBtcChat {
    id: string,
    message: string | undefined,
    sender : string | undefined,
    amountOfSats: number,
    amountInFiat: number,
    currency: string,
    dateCreated: string,
}
