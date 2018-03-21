export interface Transaction {
  id: number,
  asset_type: string,
  asset_denom: string,
  value: number,
  transaction_time: string,
  accounting_period_id: number,
  balance: number,
  message: string,
  business_ends: string 
}
