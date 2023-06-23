FROM node:latest as builder
WORKDIR /app
COPY . .
RUN npm run build


FROM node:slim
WORKDIR /app
ENV PORT 6789

COPY --from=builder /app/package.json .
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules

EXPOSE $PORT
CMD node -r dotenv/config build
