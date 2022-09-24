import { Provider } from "@ethersproject/providers";
import { Signer } from "ethers";
import {
    InputFacet,
    InputFacet__factory,
    OutputFacet,
    OutputFacet__factory,
    ERC20PortalFacet,
    ERC20PortalFacet__factory,
} from "@cartesi/rollups";

export interface Args {
    dapp: string;
    address?: string;
    addressFile?: string;
}

interface Contracts {
    inputContract: InputFacet;
    outputContract: OutputFacet;
    erc20Portal: ERC20PortalFacet;
}

const contractAddress = '0xF119CC4Ed90379e5E0CC2e5Dd1c8F8750BAfC812';

/**
 * Connect to instance of Rollups application
 * @param chainId number of chain id of connected network
 * @param provider provider or signer of connected network
 * @param args args for connection logic
 * @returns Connected rollups contracts
 */
 export const cartesiRollups = async (
    provider: Provider | Signer
): Promise<Contracts> => {
    // connect to contracts
    const inputContract = InputFacet__factory.connect(contractAddress, provider);
    const outputContract = OutputFacet__factory.connect(contractAddress, provider);
    const erc20Portal = ERC20PortalFacet__factory.connect(contractAddress, provider);
    return {
        inputContract,
        outputContract,
        erc20Portal,
    };
};