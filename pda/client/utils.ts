import * as borsh from "borsh";

export enum InstructionType{
    SetFavorites,
    InitFavorites
}

export class Favorites{

    data: (string | null)[];

    constructor({data}:{data: (string | null)[]}){
        this.data = data;
    }

    static getDeserializedData = (buffer:Buffer) => {
        const favorites = borsh.deserialize({
            struct:{
                data:{
                    array:{
                        type:{
                            option:"string"
                        },
                    }
                }
            }
        }, buffer) as Favorites;
        return favorites;
    }
}

export class FavoritesInstruction extends Favorites{

    variant: InstructionType;

    static setFavoritesSchema: borsh.Schema = {
        struct:{
            variant: 'u8', // It is important to set the variant first as we split the variant first in our contract 
            data: {
                array:{
                    type:{
                        option:'string',
                    },
                }
            },
        }
    }

    static initFavoritesSchema : borsh.Schema = {
        struct:{
            variant : 'u8'
        }
    }

    constructor(variant: InstructionType,  data: (string | null)[]){
        super({data});
        this.variant = variant;
    }
    
    getSerializedIxDataForSetFavorites(){
        return borsh.serialize(FavoritesInstruction.setFavoritesSchema, this);
    }

    getSerializedIxDataForInitFavorites(){
        return borsh.serialize(FavoritesInstruction.initFavoritesSchema, this);
    }

}