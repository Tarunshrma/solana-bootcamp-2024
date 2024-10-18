// /** @type {import('ts-jest').JestConfigWithTsJest} **/
// module.exports = {
//   testEnvironment: "node",
//   transform: {
//     "^.+.tsx?$": ["ts-jest",{}],
//   },
// };
/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  testEnvironment: "node",
  transform: {
    "^.+\\.tsx?$": "ts-jest", // Handles TypeScript files
    "^.+\\.[tj]sx?$": "babel-jest", // Handles JS and TS files via Babel
  },
  transformIgnorePatterns: [
    "/node_modules/(?!chai)", // Tells Jest not to ignore chai and to transform it
  ],
};
