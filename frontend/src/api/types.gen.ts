// This file is auto-generated by @hey-api/openapi-ts

export type Test = {
    id: number;
    title: string;
    body: string;
};

export type LoginRequest = {
    username: string;
    password: string;
};

export type ListTestEntriesData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/test';
};

export type ListTestEntriesResponses = {
    200: Array<Test>;
};

export type ListTestEntriesResponse = ListTestEntriesResponses[keyof ListTestEntriesResponses];

export type AuthLoginData = {
    body: LoginRequest;
    path?: never;
    query?: never;
    url: '/login';
};

export type AuthLoginErrors = {
    default: unknown;
};

export type AuthLoginResponses = {
    200: string;
};

export type AuthLoginResponse = AuthLoginResponses[keyof AuthLoginResponses];

export type AuthSignupData = {
    body: LoginRequest;
    path?: never;
    query?: never;
    url: '/signup';
};

export type AuthSignupErrors = {
    default: unknown;
};

export type AuthSignupResponses = {
    200: unknown;
};

export type ClientOptions = {
    baseURL: 'http://localhost:8000' | (string & {});
};