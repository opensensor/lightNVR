export interface ContractProxy {directory:string;baseURL:string;requests:{method:string;url:string;status:number}[];stop(options?:{failed?:boolean}):Promise<void>}
export declare function startProxy(target:string,options?:{prefix?:string}):Promise<ContractProxy>;
