import { ObjectType, Field, Int } from 'type-graphql';
import { getModelForClass, modelOptions } from '@typegoose/typegoose';

@modelOptions({ schemaOptions: { collection: 'users', id: false } })
@ObjectType()
export class User {
    @Field((_type) => Int)
    id: number

    @Field()
    name: string

    @Field()
    safe_name: string

    @Field((_type) => Int)
    register_time: number

    @Field((_type) => Int)
    latest_activity: number

    @Field((_type) => Int)
    privileges: number

    @Field((_type) => Int)
    silence_end: number

    @Field((_type) => [Int])
    friends: number[]

    @Field()
    country: string

    @Field((_type) => [Int])
    blocked: number[]
}

export const UserModel = getModelForClass(User);