/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

var PROTO_PATH = __dirname + '/../../../hello-tonic/proto/hellotonic/hellotonic.proto';

var grpc = require('grpc');
var protoLoader = require('@grpc/proto-loader');

var packageDefinition = protoLoader.loadSync(
  PROTO_PATH,
  {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true
  });
var hello_proto = grpc.loadPackageDefinition(packageDefinition).hellotonic;

function main() {
  var client = new hello_proto.Greeter('localhost:50003',
    grpc.credentials.createInsecure());
  var request;
  name = 'world';
  

  var i;

  console.time("startcalls");

  end = function (err, message) {
    console.log((err || !message) ? "Error" : message);
    if (--i === 1) { console.timeEnd("startcalls"); }
  };


  for (i = 0; i < 5000; i++) {
    client.sayHello({ name: name}, function (err, response) {
      console.log('Greeting:', response.message);
    },
    end);
  }
}

main();
