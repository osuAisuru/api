import { Resolver, Arg, Query, Int } from 'type-graphql';
import { User, UserModel } from '../entities/user';

@Resolver()
export class UserResolver {
    @Query((_returns) => User, { nullable: true })
    // @ts-ignore
    async userById(@Arg('id', type => Int) id: number) {
        return await UserModel.findOne({ id: id }).lean()
    }

    @Query((_returns) => User, { nullable: true })
    // @ts-ignore
    async userByName(@Arg('name') name: string) {
        return await UserModel.findOne({ name: name }).lean()
    }
}