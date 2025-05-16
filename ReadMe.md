# (Rust) Google Play Integrity Shield API

## **Objective**
Implement the **Google Play Integrity API** with a focus on the **Decode Integrity Token** operation using the Rust programming language.

The application will be hosted on Azure as an Azure Function.
See [`Quickstart: Create a Go or Rust function in Azure using Visual Studio Code`](https://learn.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other)

## **Overview**
The **Google Play Integrity API** allows app developers to detect harmful activity or untrustworthy devices interacting with their applications.  
This project enhances app security by decoding and validating the **Integrity Token** to ensure device and action integrity.

Key Highlights:
- **Token Decoding**: Parse the Integrity Token to retrieve structured data.
- **Validation**: Authenticate the token using cryptographic methods.
- **Rust Implementation**: Leverage Rust's performance and safety features for a secure solution.

---

## **Features**
1. **Fetch Integrity Token**
   - Securely receive the integrity token from the client-side application.

2. **Decode and Parse**
   - Decode the JSON Web Token (JWT) to extract its header, payload, and signature.

3. **Verify Token Authenticity**
   - Use **Google-provided public keys** to validate the token's signature.
   - Ensure the integrity of the payload and verify its authenticity.

4. **Extract Insights**
   - Retrieve important claims from the decoded token, such as:
     - Device integrity status
     - App package name validation
     - Licensing and distribution checks

---

## **Technical Stack**
- **Programming Language**: Rust
- **Core Libraries**:
  - [`jsonwebtoken`](https://crates.io/crates/jsonwebtoken): For decoding and verifying JSON Web Tokens.
  - [`reqwest`](https://crates.io/crates/reqwest): For HTTP requests to fetch Google's public keys.
  - [`serde`](https://crates.io/crates/serde): For JSON deserialization.
- **Dependencies**:
  - OpenSSL or [`ring`](https://crates.io/crates/ring) for cryptographic operations.

---

## **Workflow**

1. **Fetch Token**
   - Client-side app sends the Integrity Token to the backend.

2. **Decode the JWT**
   - Split the token into its components (header, payload, signature).
   - Decode the base64-encoded JSON sections.

3. **Verify Signature**
   - Fetch Google's public keys from their endpoint.
   - Verify the JWT signature using the appropriate public key.

4. **Validate Claims**
   - Ensure the token contains valid claims, including:
     - App package name matches the expected value.
     - Issued timestamp falls within an acceptable range.
     - Device integrity meets security standards.

5. **Return Results**
   - Provide a validation report to the client app or backend service.

---

## **Potential Challenges**
- Efficiently handling JWT signature validation in Rust.
- Securely storing and caching Google's public keys for reuse.
- Mitigating potential token tampering and handling invalid tokens gracefully.

---

## **Next Steps**
1. Create a Rust module for JWT decoding and parsing.
2. Implement signature verification using Google's public keys.
3. Build a sample app or CLI tool to demonstrate the functionality.
4. Write comprehensive test cases to ensure robustness and correctness.

---

## **Expected Outcome**
A secure and reliable Rust implementation of the Google Play Integrity API's **Decode Integrity Token** operation, ready for integration into any backend system that requires enhanced application security.
