import { ApolloServer } from 'apollo-server-express';
import { ApolloServerPluginLandingPageGraphQLPlayground } from 'apollo-server-core';
import Express from 'express';
import 'reflect-metadata';
import { buildSchema } from 'type-graphql';
import { connect } from 'mongoose';

import { UserResolver } from './resolvers/user';

import { port, mongodb } from './config.json';

const main = async () => {
    const schema = await buildSchema({
        resolvers: [UserResolver],
        emitSchemaFile: true,
        validate: false,
    })

    const mongoose = await connect(mongodb)
    await mongoose.connection

    const server = new ApolloServer({
        schema,
        plugins: [ApolloServerPluginLandingPageGraphQLPlayground],
    })

    const app = Express()

    await server.start()

    // @ts-ignore
    server.applyMiddleware({ app })

    app.listen({ port: port }, () => {
        console.log(`Server started on http://localhost:${port}/graphql`)
    })
}

main().catch((error) => {
    console.log(error, 'error')
})