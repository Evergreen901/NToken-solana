/**
 * Information about an assets
 */
export type AssetsInfo = { |
    /**
     * The amount of first asset
     */
    amountAsset: null | u64,
    /**
     * The address of first asset
     */
    addressAsset: null | PublicKey,

    /**
     * The period of first asset
     */
    periodAsset: null | u64,
    /**
     * The asset solde of first asset
     */
    assetToSoldIntoAsset: null | u64,
    |
};