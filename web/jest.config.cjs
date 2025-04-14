module.exports = {
  testMatch: [
    '**/tests/**/*.spec.js'
  ],
  testEnvironment: 'node',
  setupFilesAfterEnv: ['./tests/setup.cjs'],
  testTimeout: 30000, // Selenium tests can take time
  transform: {
    '^.+\\.jsx?$': ['babel-jest', { configFile: './babel.config.cjs' }]
  }
};
