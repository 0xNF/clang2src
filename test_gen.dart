// DO NOT MODIFY THIS FILE
// This file contains automatically generated Dart Bindings.
// It was generated via the clang2src project, and ultimately comes from a set of annotated Rust source files
// Any modifications you make to this file will be reverted whenever this file is regenerated

import 'package:ffi/ffi.dart';

import 'dart:ffi' as ffi;

import 'dart:io' show Platform, Directory;

import 'package:path/path.dart' as path;

class liboauthtoolException implements Exception {
  final String msg;
  final int code;

  const liboauthtoolException(this.msg, this.code);
}

/* Region: C Constants */
const int C_NULL = 0;
const int C_TRUE = 1;
const int C_FALSE = 0;

/* Region: Dart Constants */

const int OAUTHTOOL_PASS = 0;

const int OAUTHTOOL_FAIL = 1;

const int OAUTHTOOL_FAIL_NULL_POINTER = 2;

/* Region: FFI Enums */

abstract class _IAsFFIInt {
  int get getValueAsInt;
}

/// Code Challenge for PKCE-enabled Authorization Code Flow
/// https://www.rfc-editor.org/rfc/rfc7636#section-4.2
/// If PKCE is not enabled, use [CodeChallengeMethod.None]

enum CodeChallengeMethod implements _IAsFFIInt {
  /// None, for when the code flow isnt PKCE enabled

  None(0),

  /// Plain: code_challenge=code_verifier

  Plain(1),

  /// Sha256: code_challenge = BASE64URL-ENCODE(SHA256(ASCII(code_verifier)))

  S256(2),
  ;

  final int value;

  const CodeChallengeMethod(this.value);

  @override
  int get getValueAsInt => value;
}

/// Specifies the different types of OAuth Flows
/// Implicit,
/// Client Credentials,
/// Authorization,
/// Authorization + PKCE,
/// Device

enum FlowType implements _IAsFFIInt {
  /// Implicit Grant

  Implicit(0),

  /// Client Credentials Grant
  /// Gives keys to an application without accessing user-specific resources

  ClientCredentials(1),

  /// Authorization Grant
  /// Allows access to user-specific resources. Usually comes with an infinite-lifespan Refresh Token

  Authorization(2),

  /// Secure Authorization grant using the PKCE extension
  /// Refresh Tokens are one-time-use

  AuthorizationPKCE(3),

  /// Flow for using a second device, which has a screen, to give access to a primary device, which does not have a screen

  Device(4),
  ;

  final int value;

  const FlowType(this.value);

  @override
  int get getValueAsInt => value;
}

/* Region: FFI Structs */

class C_Engine extends ffi.Opaque {}

class C_OAuth2Authorization extends ffi.Opaque {}

class C_OAuth2ClientCredentials extends ffi.Opaque {}

class C_OAuth2Implicit extends ffi.Opaque {}

class C_OAuth2PKCE extends ffi.Opaque {}

class C_FFIArray extends ffi.Struct {
  /// Number of elements in the returned array

  @ffi.UintPtr()
  external int len;

  /// Max size of the array

  @ffi.UintPtr()
  external int cap;

  /// pointer to the first item in the array

  external ffi.Pointer<ffi.Pointer<ffi.Char>> arr;
}

class C_TokenResponse extends ffi.Struct {
  /// If not null, contains a token that can be used to access the service

  external ffi.Pointer<ffi.Char> accessToken;

  /// If not null, contains a token that can be used to get a new access token

  external ffi.Pointer<ffi.Char> refreshToken;

  /// Seconds from received time that the token expires at

  @ffi.Int64()
  external int expiresAt;

  /// If not null, denotes what kind of token this is.  Usually Bearer

  external ffi.Pointer<ffi.Char> tokenType;

  external ffi.Pointer<C_FFIArray> scopes;
}

class C_AuthUrlOutput extends ffi.Struct {
  external ffi.Pointer<ffi.Char> url;

  external ffi.Pointer<ffi.Char> localState;

  external ffi.Pointer<ffi.Char> pkceVerifierState;
}

class C_ParsedAuthorizationCode extends ffi.Struct {
  /// Authorization Code. Always present.

  external ffi.Pointer<ffi.Char> code;

  /// State returned from server. Should match state given to server. Not always present

  external ffi.Pointer<ffi.Char> state;
}

/* Region: FFI Free Functions */

/// Encrypts a given plain text string with the engine paramaters and returns a Base64-encoded string
/// #meta_param: encrypted_output;output;string;
/// #meta_param: plain_text;string;
/// #meta_param: engine;as_ptr;
/// #meta_param: err_ptr;error;

int ffi_encrypt(
  ffi.Pointer<ffi.Char> plain_text,
  ffi.Pointer<ffi.Pointer<ffi.Char>> encrypted_output,
  ffi.Pointer<C_Engine> engine,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_encrypt(
    plain_text,
    encrypted_output,
    engine,
    err_ptr,
  );
}

final ffi_encryptPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('encrypt');
final _ffi_encrypt = ffi_encryptPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Decrypts a Base64-Encoded binary blob with the engine parameters and returns a list of bytes.
/// #meta_param: encrypted_text;string;
/// #meta_param: decrypted_output;output;string;
/// #meta_param: engine;as_ptr;
/// #meta_param: err_ptr;error;

int ffi_decrypt(
  ffi.Pointer<ffi.Char> encrypted_text,
  ffi.Pointer<ffi.Pointer<ffi.Char>> decrypted_output,
  ffi.Pointer<C_Engine> engine,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_decrypt(
    encrypted_text,
    decrypted_output,
    engine,
    err_ptr,
  );
}

final ffi_decryptPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('decrypt');
final _ffi_decrypt = ffi_decryptPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Initializes a new OAuth2PKCE manager into the `this` pointer
/// #meta_param: this_;output;this;
/// #meta_param: client_id;string;
/// #meta_param: client_secret;string;
/// #meta_param: authorization_url;url;
/// #meta_param: redirect_url;url;
/// #meta_param: token_url;url;
/// #meta_param: scopes;string;
/// #meta_param: timeout_in_milliseconds;duration;
/// #meta_param: err_ptr;error;
/// #meta_param: extra_parameters;string;

int ffi_OAuth2PKCE_new(
  ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>> this_,
  ffi.Pointer<ffi.Char> client_id,
  ffi.Pointer<ffi.Char> client_secret,
  ffi.Pointer<ffi.Char> authorization_url,
  ffi.Pointer<ffi.Char> token_url,
  ffi.Pointer<ffi.Char> redirect_url,
  CodeChallengeMethod challenge_method,
  ffi.Pointer<ffi.Char> scopes,
  ffi.Pointer<ffi.Char> extra_parameters,
  int timeout_in_milliseconds,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_new(
    this_,
    client_id,
    client_secret,
    authorization_url,
    token_url,
    redirect_url,
    challenge_method.getValueAsInt,
    scopes,
    extra_parameters,
    timeout_in_milliseconds,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_newPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Int32,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Int64,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_new');
final _ffi_OAuth2PKCE_new = ffi_OAuth2PKCE_newPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  int,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  int,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Launches a local web server and attempts to perform the token exchange automatically
/// This can only be used on devices that have a web browser
/// #meta_param: this_;this;
/// #meta_param: token_output;output;
/// #meta_param: err_ptr;error;

int ffi_OAuth2PKCE_get_token_automatic(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_get_token_automatic(
    this_,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_get_token_automaticPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_get_token_automatic');
final _ffi_OAuth2PKCE_get_token_automatic =
    ffi_OAuth2PKCE_get_token_automaticPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// #meta_param: this_;this;
/// #meta_param: redirect_url;url;
/// #meta_param: verifier_state;string;
/// #meta_param: token_output;output;
/// #meta_param: timeout_in_milliseconds;duration;
/// #meta_param: err_ptr;error;

int ffi_OAuth2PKCE_start_web_server_for_callback(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Char> redirect_url,
  ffi.Pointer<ffi.Char> verifier_state,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  int timeout_in_milliseconds,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_start_web_server_for_callback(
    this_,
    redirect_url,
    verifier_state,
    token_output,
    timeout_in_milliseconds,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_start_web_server_for_callbackPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Int64,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_start_web_server_for_callback');
final _ffi_OAuth2PKCE_start_web_server_for_callback =
    ffi_OAuth2PKCE_start_web_server_for_callbackPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  int,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Uses the initializes `this` manager to get the Authoirzation URL as known by its parameters
/// #meta_param: this_;this;
/// #meta_param: authorization_output;output;
/// #meta_param: err_ptr;error;

int ffi_OAuth2PKCE_get_authorization_url(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> authorization_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_get_authorization_url(
    this_,
    authorization_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_get_authorization_urlPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_get_authorization_url');
final _ffi_OAuth2PKCE_get_authorization_url =
    ffi_OAuth2PKCE_get_authorization_urlPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Exchanges an Authorization Code for an Access/Refresh Token
/// If Verifier State is null, defaults to the using the verifier state known to the OAuth2Pkce manager
/// If Verifier State is null, overwrites the verifier state to that one instead
/// #meta_param: this_;this;
/// #meta_param: authorization_code;string;
/// #meta_param: verifier_state;string;
/// #meta_param: token_output;output;
/// #meta_param: err_ptr;error;

int ffi_OAuth2PKCE_exchange_authorization_code_for_token(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Char> authorization_code,
  ffi.Pointer<ffi.Char> verifier_state,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_exchange_authorization_code_for_token(
    this_,
    authorization_code,
    verifier_state,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_exchange_authorization_code_for_tokenPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_exchange_authorization_code_for_token');
final _ffi_OAuth2PKCE_exchange_authorization_code_for_token =
    ffi_OAuth2PKCE_exchange_authorization_code_for_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Given a Refresh Token, use it to retrieve a new TokenResponse
/// #meta_param: this_;this;
/// #meta_param: err_ptr;error;
/// #meta_param: token_output;output;
/// #meta_param: refresh_token;string;

int ffi_OAuth2PKCE_refresh_access_token(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Char> refresh_token,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_refresh_access_token(
    this_,
    refresh_token,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_refresh_access_tokenPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_refresh_access_token');
final _ffi_OAuth2PKCE_refresh_access_token =
    ffi_OAuth2PKCE_refresh_access_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Initializes a new OAuth2Authoirzation manager into the `this` pointer
/// #meta_param: this_;output;this;
/// #meta_param: client_id;string;
/// #meta_param: client_secret;string;
/// #meta_param: authorization_url;url;
/// #meta_param: redirect_url;url;
/// #meta_param: token_url;url;
/// #meta_param: scopes;string;
/// #meta_param: timeout_in_milliseconds;duration;
/// #meta_param: err_ptr;error;
/// #meta_param: extra_parameters;string;

int ffi_OAuth2Authorization_new(
  ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>> this_,
  ffi.Pointer<ffi.Char> client_id,
  ffi.Pointer<ffi.Char> client_secret,
  ffi.Pointer<ffi.Char> authorization_url,
  ffi.Pointer<ffi.Char> token_url,
  ffi.Pointer<ffi.Char> redirect_url,
  ffi.Pointer<ffi.Char> scopes,
  ffi.Pointer<ffi.Char> extra_parameters,
  int timeout_in_milliseconds,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_new(
    this_,
    client_id,
    client_secret,
    authorization_url,
    token_url,
    redirect_url,
    scopes,
    extra_parameters,
    timeout_in_milliseconds,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_newPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Int64,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_new');
final _ffi_OAuth2Authorization_new = ffi_OAuth2Authorization_newPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  int,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Uses the initializes `this` manager to get the Authoirzation URL as known by its parameters
/// #meta_param: this_;this;
/// #meta_param: authorization_output;output;
/// #meta_param: err_ptr;error;

int ffi_OAuth2Authorization_get_authorization_url(
  ffi.Pointer<C_OAuth2Authorization> this_,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> authorization_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_get_authorization_url(
    this_,
    authorization_output,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_get_authorization_urlPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_get_authorization_url');
final _ffi_OAuth2Authorization_get_authorization_url =
    ffi_OAuth2Authorization_get_authorization_urlPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Exchanges an Authorization Code for an Access/Refresh Token
/// If Verifier State is null, defaults to the using the verifier state known to the OAuth2Pkce manager
/// If Verifier State is null, overwrites the verifier state to that one instead
/// #meta_param: this_;this;
/// #meta_param: authorization_code;string;
/// #meta_param: token_output;output;
/// #meta_param: err_ptr;error;

int ffi_OAuth2Authorization_exchange_authorization_code_for_token(
  ffi.Pointer<C_OAuth2Authorization> this_,
  ffi.Pointer<ffi.Char> authorization_code,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_exchange_authorization_code_for_token(
    this_,
    authorization_code,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_exchange_authorization_code_for_tokenPtr =
    _lookup<
        ffi.NativeFunction<
            ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_exchange_authorization_code_for_token');
final _ffi_OAuth2Authorization_exchange_authorization_code_for_token =
    ffi_OAuth2Authorization_exchange_authorization_code_for_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Given a Refresh Token, use it to retrieve a new TokenResponse
/// #meta_param: this_;this;
/// #meta_param: refresh_token;string;
/// #meta_param: token_output;output;
/// #meta_param: err_ptr;error;

int ffi_OAuth2Authorization_refresh_access_token(
  ffi.Pointer<C_OAuth2Authorization> this_,
  ffi.Pointer<ffi.Char> refresh_token,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_refresh_access_token(
    this_,
    refresh_token,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_refresh_access_tokenPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_refresh_access_token');
final _ffi_OAuth2Authorization_refresh_access_token =
    ffi_OAuth2Authorization_refresh_access_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Given a filled in callback URL (aka 'https://example.com/callback?state=123&code=abc),
/// returns a ParsedAuthorizationCode object containing the state and code
/// #meta_param: filled_callback_url;string;
/// #meta_param: parsed_authorization_code_output;output;
/// #meta_param: err_ptr;error;

int ffi_parse_authorization_callback_url(
  ffi.Pointer<ffi.Char> filled_callback_url,
  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>
      parsed_authorization_code_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_parse_authorization_callback_url(
    filled_callback_url,
    parsed_authorization_code_output,
    err_ptr,
  );
}

final ffi_parse_authorization_callback_urlPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('parse_authorization_callback_url');
final _ffi_parse_authorization_callback_url =
    ffi_parse_authorization_callback_urlPtr.asFunction<
        int Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Initializes a new Engine to the given pointer
/// #meta_param: engine;this;output;
/// #meta_param: err_ptr;error;

int ffi_Engine_new(
  ffi.Pointer<ffi.Pointer<C_Engine>> engine,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_Engine_new(
    engine,
    err_ptr,
  );
}

final ffi_Engine_newPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Pointer<C_Engine>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('Engine_new');
final _ffi_Engine_new = ffi_Engine_newPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Pointer<C_Engine>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

/// Frees memory used by a Engine instance
/// #meta_param: engine;this;

void ffi_Engine_free(
  ffi.Pointer<C_Engine> engine,
) {
  return _ffi_Engine_free(
    engine,
  );
}

final ffi_Engine_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_Engine>,
)>>('Engine_free');
final _ffi_Engine_free = ffi_Engine_freePtr.asFunction<
    void Function(
  ffi.Pointer<C_Engine>,
)>();

/// Frees memory used by an OAuthManagerPKCE instance
/// #meta_param: mgr;this;

void ffi_OAuth2PKCE_free(
  ffi.Pointer<C_OAuth2PKCE> mgr,
) {
  return _ffi_OAuth2PKCE_free(
    mgr,
  );
}

final ffi_OAuth2PKCE_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2PKCE>,
)>>('OAuth2PKCE_free');
final _ffi_OAuth2PKCE_free = ffi_OAuth2PKCE_freePtr.asFunction<
    void Function(
  ffi.Pointer<C_OAuth2PKCE>,
)>();

/// Frees memory used by an OAuthManagerAuthorization instance
/// #meta_param: mgr;this;

void ffi_OAuth2Authorization_free(
  ffi.Pointer<C_OAuth2Authorization> mgr,
) {
  return _ffi_OAuth2Authorization_free(
    mgr,
  );
}

final ffi_OAuth2Authorization_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2Authorization>,
)>>('OAuth2Authorization_free');
final _ffi_OAuth2Authorization_free =
    ffi_OAuth2Authorization_freePtr.asFunction<
        void Function(
  ffi.Pointer<C_OAuth2Authorization>,
)>();

/// Frees memory used by an OAuthManagerClientCredentials instance
/// #meta_param: mgr;this;

void ffi_OAuth2ClientCredentials_free(
  ffi.Pointer<C_OAuth2ClientCredentials> mgr,
) {
  return _ffi_OAuth2ClientCredentials_free(
    mgr,
  );
}

final ffi_OAuth2ClientCredentials_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2ClientCredentials>,
)>>('OAuth2ClientCredentials_free');
final _ffi_OAuth2ClientCredentials_free =
    ffi_OAuth2ClientCredentials_freePtr.asFunction<
        void Function(
  ffi.Pointer<C_OAuth2ClientCredentials>,
)>();

/// Frees memory used by an OAuthManagerImplicit instance
/// #meta_param: mgr;this;

void ffi_OAuth2Implicit_free(
  ffi.Pointer<C_OAuth2Implicit> mgr,
) {
  return _ffi_OAuth2Implicit_free(
    mgr,
  );
}

final ffi_OAuth2Implicit_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2Implicit>,
)>>('OAuth2Implicit_free');
final _ffi_OAuth2Implicit_free = ffi_OAuth2Implicit_freePtr.asFunction<
    void Function(
  ffi.Pointer<C_OAuth2Implicit>,
)>();

/// Function to test deserialization of strings across FFI boundaries

ffi.Pointer<C_FFIArray> ffi_TestGetRustStringList() {
  return _ffi_TestGetRustStringList();
}

final ffi_TestGetRustStringListPtr =
    _lookup<ffi.NativeFunction<ffi.Pointer<C_FFIArray> Function()>>(
        'TestGetRustStringList');
final _ffi_TestGetRustStringList = ffi_TestGetRustStringListPtr
    .asFunction<ffi.Pointer<C_FFIArray> Function()>();

/* Region: Dart Classes for use by the end-user */

class Engine implements _IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_Engine> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_Engine_freePtr.cast());

  /* Constructors */

  Engine._fromCPtr(
    this._selfPtr,
  ) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_Engine>>

  factory Engine._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_Engine>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return Engine._fromCPtr(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }

  /// Initializes a new Engine to the given pointer
  /// #meta_param: engine;this;output;
  /// #meta_param: err_ptr;error;

  factory Engine.newCreate() {
    final cEngine_newOutputPtr = _getPointerForType<Engine>();

    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
        _getPointerForType<String>().cast();

    /* call native function */
    int errCode = ffi_Engine_new(
      cEngine_newOutputPtr.cast(),
      cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
      /* free pointers if required  */

      calloc.free(cEngine_newOutputPtr);

      /* throw final Exception */
      throw liboauthtoolException(
          _getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */

    calloc.free(cErrPtr);

    /* return final value */
    return Engine._fromCPointerPointer(cEngine_newOutputPtr.cast());
    'todo: create constructor boi';
  }
}

class OAuth2Authorization implements _IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2Authorization> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2Authorization_freePtr.cast());

  /* Constructors */

  OAuth2Authorization._fromCPtr(
    this._selfPtr,
  ) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_OAuth2Authorization>>

  factory OAuth2Authorization._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2Authorization._fromCPtr(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }

  /// Initializes a new OAuth2Authoirzation manager into the `this` pointer
  /// #meta_param: this_;output;this;
  /// #meta_param: client_id;string;
  /// #meta_param: client_secret;string;
  /// #meta_param: authorization_url;url;
  /// #meta_param: redirect_url;url;
  /// #meta_param: token_url;url;
  /// #meta_param: scopes;string;
  /// #meta_param: timeout_in_milliseconds;duration;
  /// #meta_param: err_ptr;error;
  /// #meta_param: extra_parameters;string;

  factory OAuth2Authorization.newCreate(
    String client_id,
    String client_secret,
    Uri authorization_url,
    Uri token_url,
    Uri redirect_url,
    String scopes,
    String extra_parameters,
    Duration timeout_in_milliseconds,
  ) {
    final cOAuth2Authorization_newOutputPtr =
        _getPointerForType<OAuth2Authorization>();

    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
        _getPointerForType<String>().cast();

    final dynamic cclient_idForFFI = _transformToFFI(client_id);

    final dynamic cclient_secretForFFI = _transformToFFI(client_secret);

    final dynamic cauthorization_urlForFFI = _transformToFFI(authorization_url);

    final dynamic ctoken_urlForFFI = _transformToFFI(token_url);

    final dynamic credirect_urlForFFI = _transformToFFI(redirect_url);

    final dynamic cscopesForFFI = _transformToFFI(scopes);

    final dynamic cextra_parametersForFFI = _transformToFFI(extra_parameters);

    final dynamic ctimeout_in_millisecondsForFFI =
        _transformToFFI(timeout_in_milliseconds);

    /* call native function */
    int errCode = ffi_OAuth2Authorization_new(
      cOAuth2Authorization_newOutputPtr.cast(),
      cclient_idForFFI,
      cclient_secretForFFI,
      cauthorization_urlForFFI,
      ctoken_urlForFFI,
      credirect_urlForFFI,
      cscopesForFFI,
      cextra_parametersForFFI,
      ctimeout_in_millisecondsForFFI,
      cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
      /* free pointers if required  */

      calloc.free(cclient_idForFFI);

      calloc.free(cclient_secretForFFI);

      calloc.free(cauthorization_urlForFFI);

      calloc.free(ctoken_urlForFFI);

      calloc.free(credirect_urlForFFI);

      calloc.free(cscopesForFFI);

      calloc.free(cextra_parametersForFFI);

      calloc.free(cOAuth2Authorization_newOutputPtr);

      /* throw final Exception */
      throw liboauthtoolException(
          _getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */

    calloc.free(cclient_idForFFI);

    calloc.free(cclient_secretForFFI);

    calloc.free(cauthorization_urlForFFI);

    calloc.free(ctoken_urlForFFI);

    calloc.free(credirect_urlForFFI);

    calloc.free(cscopesForFFI);

    calloc.free(cextra_parametersForFFI);

    calloc.free(cErrPtr);

    /* return final value */
    return OAuth2Authorization._fromCPointerPointer(
        cOAuth2Authorization_newOutputPtr.cast());
    'todo: create constructor boi';
  }
}

class OAuth2ClientCredentials implements _IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2ClientCredentials> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2ClientCredentials_freePtr.cast());

  /* Constructors */

  OAuth2ClientCredentials._fromCPtr(
    this._selfPtr,
  ) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_OAuth2ClientCredentials>>

  factory OAuth2ClientCredentials._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2ClientCredentials>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2ClientCredentials._fromCPtr(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class OAuth2Implicit implements _IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2Implicit> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2Implicit_freePtr.cast());

  /* Constructors */

  OAuth2Implicit._fromCPtr(
    this._selfPtr,
  ) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_OAuth2Implicit>>

  factory OAuth2Implicit._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2Implicit>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2Implicit._fromCPtr(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class OAuth2PKCE implements _IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2PKCE> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2PKCE_freePtr.cast());

  /* Constructors */

  OAuth2PKCE._fromCPtr(
    this._selfPtr,
  ) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_OAuth2PKCE>>

  factory OAuth2PKCE._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2PKCE._fromCPtr(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }

  /// Initializes a new OAuth2PKCE manager into the `this` pointer
  /// #meta_param: this_;output;this;
  /// #meta_param: client_id;string;
  /// #meta_param: client_secret;string;
  /// #meta_param: authorization_url;url;
  /// #meta_param: redirect_url;url;
  /// #meta_param: token_url;url;
  /// #meta_param: scopes;string;
  /// #meta_param: timeout_in_milliseconds;duration;
  /// #meta_param: err_ptr;error;
  /// #meta_param: extra_parameters;string;

  factory OAuth2PKCE.newCreate(
    String client_id,
    String client_secret,
    Uri authorization_url,
    Uri token_url,
    Uri redirect_url,
    CodeChallengeMethod challenge_method,
    String scopes,
    String extra_parameters,
    Duration timeout_in_milliseconds,
  ) {
    final cOAuth2PKCE_newOutputPtr = _getPointerForType<OAuth2PKCE>();

    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
        _getPointerForType<String>().cast();

    final dynamic cclient_idForFFI = _transformToFFI(client_id);

    final dynamic cclient_secretForFFI = _transformToFFI(client_secret);

    final dynamic cauthorization_urlForFFI = _transformToFFI(authorization_url);

    final dynamic ctoken_urlForFFI = _transformToFFI(token_url);

    final dynamic credirect_urlForFFI = _transformToFFI(redirect_url);

    final dynamic cchallenge_methodForFFI = _transformToFFI(challenge_method);

    final dynamic cscopesForFFI = _transformToFFI(scopes);

    final dynamic cextra_parametersForFFI = _transformToFFI(extra_parameters);

    final dynamic ctimeout_in_millisecondsForFFI =
        _transformToFFI(timeout_in_milliseconds);

    /* call native function */
    int errCode = ffi_OAuth2PKCE_new(
      cOAuth2PKCE_newOutputPtr.cast(),
      cclient_idForFFI,
      cclient_secretForFFI,
      cauthorization_urlForFFI,
      ctoken_urlForFFI,
      credirect_urlForFFI,
      cchallenge_methodForFFI,
      cscopesForFFI,
      cextra_parametersForFFI,
      ctimeout_in_millisecondsForFFI,
      cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
      /* free pointers if required  */

      calloc.free(cclient_idForFFI);

      calloc.free(cclient_secretForFFI);

      calloc.free(cauthorization_urlForFFI);

      calloc.free(ctoken_urlForFFI);

      calloc.free(credirect_urlForFFI);

      calloc.free(cscopesForFFI);

      calloc.free(cextra_parametersForFFI);

      calloc.free(cOAuth2PKCE_newOutputPtr);

      /* throw final Exception */
      throw liboauthtoolException(
          _getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */

    calloc.free(cclient_idForFFI);

    calloc.free(cclient_secretForFFI);

    calloc.free(cauthorization_urlForFFI);

    calloc.free(ctoken_urlForFFI);

    calloc.free(credirect_urlForFFI);

    calloc.free(cscopesForFFI);

    calloc.free(cextra_parametersForFFI);

    calloc.free(cErrPtr);

    /* return final value */
    return OAuth2PKCE._fromCPointerPointer(cOAuth2PKCE_newOutputPtr.cast());
    'todo: create constructor boi';
  }
}

class FFIArray {
  /* Fields */

  /// Number of elements in the returned array

  final int len;

  /// Max size of the array

  final int cap;

  /// pointer to the first item in the array

  final List<String> arr;

  /* Constructors */

  FFIArray._fromFields(
    this.len,
    this.cap,
    this.arr,
  );

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_FFIArray>>

  factory FFIArray._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_FFIArray>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return FFIArray._fromCStruct(st);
  }

  /// Creates an instance of this class from a struct reference

  factory FFIArray._fromCStruct(
    C_FFIArray c,
  ) {
    final int _clen = _transformFromFFI(
      c.len,
    );

    final int _ccap = _transformFromFFI(
      c.cap,
    );

    final List<String> _carr = _transformFromFFI(
      c.arr,
      isList: true,
    );

    final _FFIArrayRet = FFIArray._fromFields(
      _clen,
      _ccap,
      _carr,
    );
    return _FFIArrayRet;
  }
}

class TokenResponse {
  /* Fields */

  /// If not null, contains a token that can be used to access the service

  final String accessToken;

  /// If not null, contains a token that can be used to get a new access token

  final String? refreshToken;

  /// Seconds from received time that the token expires at

  final int expiresAt;

  /// If not null, denotes what kind of token this is.  Usually Bearer

  final String? tokenType;

  final FFIArray scopes;

  /* Constructors */

  TokenResponse._fromFields(
    this.accessToken,
    this.refreshToken,
    this.expiresAt,
    this.tokenType,
    this.scopes,
  );

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_TokenResponse>>

  factory TokenResponse._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return TokenResponse._fromCStruct(st);
  }

  /// Creates an instance of this class from a struct reference

  factory TokenResponse._fromCStruct(
    C_TokenResponse c,
  ) {
    final String _caccessToken = _transformFromFFI(
      c.accessToken,
    );

    final String? _crefreshToken = _transformFromFFI(
      c.refreshToken,
    );

    final int _cexpiresAt = _transformFromFFI(
      c.expiresAt,
    );

    final String? _ctokenType = _transformFromFFI(
      c.tokenType,
    );

    final FFIArray _cscopes = _transformFromFFI(
      c.scopes,
    );

    final _TokenResponseRet = TokenResponse._fromFields(
      _caccessToken,
      _crefreshToken,
      _cexpiresAt,
      _ctokenType,
      _cscopes,
    );
    return _TokenResponseRet;
  }
}

class AuthUrlOutput {
  /* Fields */

  final Uri url;

  final String? localState;

  final String? pkceVerifierState;

  /* Constructors */

  AuthUrlOutput._fromFields(
    this.url,
    this.localState,
    this.pkceVerifierState,
  );

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_AuthUrlOutput>>

  factory AuthUrlOutput._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return AuthUrlOutput._fromCStruct(st);
  }

  /// Creates an instance of this class from a struct reference

  factory AuthUrlOutput._fromCStruct(
    C_AuthUrlOutput c,
  ) {
    final Uri _curl = _transformFromFFI(
      c.url,
      isUri: true,
    );

    final String? _clocalState = _transformFromFFI(
      c.localState,
    );

    final String? _cpkceVerifierState = _transformFromFFI(
      c.pkceVerifierState,
    );

    final _AuthUrlOutputRet = AuthUrlOutput._fromFields(
      _curl,
      _clocalState,
      _cpkceVerifierState,
    );
    return _AuthUrlOutputRet;
  }
}

class ParsedAuthorizationCode {
  /* Fields */

  /// Authorization Code. Always present.

  final String code;

  /// State returned from server. Should match state given to server. Not always present

  final String? state;

  /* Constructors */

  ParsedAuthorizationCode._fromFields(
    this.code,
    this.state,
  );

  /* Functions */

  /// Creates an instance of this class from a Pointer<Pointer<C_ParsedAuthorizationCode>>

  factory ParsedAuthorizationCode._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return ParsedAuthorizationCode._fromCStruct(st);
  }

  /// Creates an instance of this class from a struct reference

  factory ParsedAuthorizationCode._fromCStruct(
    C_ParsedAuthorizationCode c,
  ) {
    final String _ccode = _transformFromFFI(
      c.code,
    );

    final String? _cstate = _transformFromFFI(
      c.state,
    );

    final _ParsedAuthorizationCodeRet = ParsedAuthorizationCode._fromFields(
      _ccode,
      _cstate,
    );
    return _ParsedAuthorizationCodeRet;
  }
}

/* Region: Dart Free Functions */

String encrypt(
  String plain_text,
  Engine engine,
) {
  /* Get error pointer in case function returns failure */
  final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
      _getPointerForType<String>().cast();

  /* get pointer types for items that require it*/

  final cplain_textPtr = _getPointerForData(plain_text);

  final cenginePtr = _getPointerForData(engine);

  /* get Output Pointer type */
  final cencryptOutputPtr = _getPointerForType<String>();

  /* call native function */
  int errCode = ffi_encrypt(
    cencryptOutputPtr.cast(),
    cplain_textPtr.cast(),
    cenginePtr.cast(),
    cErrPtr,
  );

  /* Check error code */
  if (errCode != C_FALSE) {
    /* free pointers if required  */

    calloc.free(cplain_textPtr);

    calloc.free(cencryptOutputPtr);

    /* throw final Exception */
    throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
  }

  /* Free allocated pointers */

  calloc.free(cplain_textPtr);

  calloc.free(cErrPtr);

  /* return final value */
  return _transformFromFFI(cencryptOutputPtr);
}

String decrypt(
  String encrypted_text,
  Engine engine,
) {
  /* Get error pointer in case function returns failure */
  final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
      _getPointerForType<String>().cast();

  /* get pointer types for items that require it*/

  final cencrypted_textPtr = _getPointerForData(encrypted_text);

  final cenginePtr = _getPointerForData(engine);

  /* get Output Pointer type */
  final cdecryptOutputPtr = _getPointerForType<String>();

  /* call native function */
  int errCode = ffi_decrypt(
    cdecryptOutputPtr.cast(),
    cencrypted_textPtr.cast(),
    cenginePtr.cast(),
    cErrPtr,
  );

  /* Check error code */
  if (errCode != C_FALSE) {
    /* free pointers if required  */

    calloc.free(cencrypted_textPtr);

    calloc.free(cdecryptOutputPtr);

    /* throw final Exception */
    throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
  }

  /* Free allocated pointers */

  calloc.free(cencrypted_textPtr);

  calloc.free(cErrPtr);

  /* return final value */
  return _transformFromFFI(cdecryptOutputPtr);
}

ParsedAuthorizationCode parse_authorization_callback_url(
  String filled_callback_url,
) {
  /* Get error pointer in case function returns failure */
  final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
      _getPointerForType<String>().cast();

  /* get pointer types for items that require it*/

  final cfilled_callback_urlPtr = _getPointerForData(filled_callback_url);

  /* get Output Pointer type */
  final cparse_authorization_callback_urlOutputPtr =
      _getPointerForType<ParsedAuthorizationCode>();

  /* call native function */
  int errCode = ffi_parse_authorization_callback_url(
    cparse_authorization_callback_urlOutputPtr.cast(),
    cfilled_callback_urlPtr.cast(),
    cErrPtr,
  );

  /* Check error code */
  if (errCode != C_FALSE) {
    /* free pointers if required  */

    calloc.free(cfilled_callback_urlPtr);

    calloc.free(cparse_authorization_callback_urlOutputPtr);

    /* throw final Exception */
    throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
  }

  /* Free allocated pointers */

  calloc.free(cfilled_callback_urlPtr);

  calloc.free(cErrPtr);

  /* return final value */
  return _transformFromFFI(cparse_authorization_callback_urlOutputPtr);
}


/* Region: Dart Pointer Utility Functions  */

/// Interface to get a Pointer to the backing data on classes that
/// cross FFI boundaries
abstract class _IWithPtr {
  ffi.Pointer<ffi.Void> getPointer();
}

/// Holds the symbol lookup function.
final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
    _lookup = loadLibrary(getLibraryPath()).lookup;

dynamic _transformToFFI<T>(T upperData) {
  if (T == String) {
    return _stringToFFIPointer(upperData as String);
  } else if (T == int || T == double) {
    return upperData;
  } else if (T == bool) {
    return upperData as bool;
  } else if (T == Uri) {
    final s = (upperData as Uri).toString();
    return _stringToFFIPointer(s);
  } else if (T == Duration) {
    return (upperData as Duration).inMilliseconds;
  } else if (T == DateTime) {
    return (upperData as DateTime).millisecondsSinceEpoch;
  } else if (T == List<String>) {
    // TODO(nf, 04/20/23): This join is only valid for OAuth Space-Separated lists.
    // It is not general.
    final s = (upperData as List<String>).join(' ');
    return _stringToFFIPointer(s);
  } else if (T == Map<String, String>) {
    // TODO(nf, 04/20/23): This is only valid for String:String pairs
    // It is not
    return _hashMapToFFIPointer(upperData as Map<String, String>);
  } else if (T == _IWithPtr) {
    return _getPointerForData(upperData);
  } else if (T == ffi.Pointer) {
    return upperData as ffi.Pointer;
  } else if (upperData is _IAsFFIInt) {
    return (upperData as _IAsFFIInt);
  } else {
    throw liboauthtoolException(
        'Invalid ffi data, cannot transform into FFI for for $T', -5);
  }
}

dynamic _transformFromFFI(dynamic data,
    {bool isList = false,
    bool isHashMap = false,
    bool isUri = false,
    bool isDuration = false,
    bool isDateTime = false}) {
  if (data is int || data is double) {
    if (isDuration) {
      return Duration(milliseconds: data);
    } else if (isDateTime) {
      return DateTime.fromMillisecondsSinceEpoch(data);
    } else {
      return data;
    }
  } else if (data is bool) {
    return data;
  } else if (data is String) {
    return data;
  } else if (data is ffi.Pointer) {
    if (data.address == ffi.nullptr.address) {
      return null;
    } else if (isList) {
      throw Exception('Lists not yet implemented');
    } else if (data is ffi.Pointer<ffi.Pointer<ffi.Char>> ||
        data is ffi.Pointer<ffi.Char>) {
      late final String s;
      if (data is ffi.Pointer<ffi.Pointer<ffi.Char>>) {
        s = _getDartStringFromDoublePtr(data);
      } else {
        s = _getDartStringFromPtr(data as ffi.Pointer<ffi.Char>);
      }
      if (isUri) {
        return Uri.parse(s);
      } else {
        return s;
      }
    } else if (data is ffi.Pointer<ffi.Pointer<ffi.NativeType>>) {
      return transformFromPointer(data);
    }
  }
  throw liboauthtoolException('Invalid data in transformFromFFI: $data', -4);
}

T transformFromPointer<T, E extends ffi.NativeType>(
    ffi.Pointer<ffi.Pointer<E>> data) {
  if (T == String) {
    return _getDartStringFromDoublePtr(data.cast()) as T;
  } else if (T == C_Engine) {
    return Engine._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2Authorization) {
    return OAuth2Authorization._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2ClientCredentials) {
    return OAuth2ClientCredentials._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2Implicit) {
    return OAuth2Implicit._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2PKCE) {
    return OAuth2PKCE._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_FFIArray) {
    return FFIArray._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_TokenResponse) {
    return TokenResponse._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_AuthUrlOutput) {
    return AuthUrlOutput._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_ParsedAuthorizationCode) {
    return ParsedAuthorizationCode._fromCPointerPointer(data.cast()) as T;
  }

  throw liboauthtoolException('Invalid data in transformFromPointer: $T', -4);
}

ffi.Pointer<ffi.Void> _getPointerForType<T>() {
  if (T == String) {
    return _getEmptyStringPointer().cast();
  } else if (T == Engine || T == C_Engine) {
    return calloc<ffi.Pointer<C_Engine>>().cast();
  } else if (T == OAuth2Authorization || T == C_OAuth2Authorization) {
    return calloc<ffi.Pointer<C_OAuth2Authorization>>().cast();
  } else if (T == OAuth2ClientCredentials || T == C_OAuth2ClientCredentials) {
    return calloc<ffi.Pointer<C_OAuth2ClientCredentials>>().cast();
  } else if (T == OAuth2Implicit || T == C_OAuth2Implicit) {
    return calloc<ffi.Pointer<C_OAuth2Implicit>>().cast();
  } else if (T == OAuth2PKCE || T == C_OAuth2PKCE) {
    return calloc<ffi.Pointer<C_OAuth2PKCE>>().cast();
  } else if (T == FFIArray || T == C_FFIArray) {
    return calloc<ffi.Pointer<C_FFIArray>>().cast();
  } else if (T == TokenResponse || T == C_TokenResponse) {
    return calloc<ffi.Pointer<C_TokenResponse>>().cast();
  } else if (T == AuthUrlOutput || T == C_AuthUrlOutput) {
    return calloc<ffi.Pointer<C_AuthUrlOutput>>().cast();
  } else if (T == ParsedAuthorizationCode || T == C_ParsedAuthorizationCode) {
    return calloc<ffi.Pointer<C_ParsedAuthorizationCode>>().cast();
  } else {
    throw liboauthtoolException('Invalid type: $T', -3);
  }
}

/// Returns a castable pointer based on the input data.
/// This function is only valid for Types [String, {custom C generated classes}]
/// Will throw an Exception if passed invalid types
ffi.Pointer<ffi.Void> _getPointerForData(dynamic data) {
  if (data is String) {
    return _stringToFFIPointer(data).cast();
  } else if (data is _IWithPtr) {
    return data.getPointer().cast();
  } else {
    throw liboauthtoolException('Invalid data type for pointer: $data', -2);
  }
}

/// Returns a Dart String from a `char*`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For double pointers `char**` use `_getDartStringFromDoublePtr`
String _getDartStringFromPtr(ffi.Pointer<ffi.Char> charPtr) {
  final asUtf8Ptr = charPtr.cast<Utf8>();
  final asDartString = asUtf8Ptr.toDartString();
  calloc.free(charPtr);
  return asDartString;
}

/// Returns a Dart String from a `char**`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For single pointers `char*` use `_getDartStringFromPtr`
String _getDartStringFromDoublePtr(
    ffi.Pointer<ffi.Pointer<ffi.Char>> doublePtr) {
  final asCharPtr = doublePtr.value.cast<ffi.Char>();
  final dstr = _getDartStringFromPtr(asCharPtr);
  calloc.free(doublePtr);
  return dstr;
}

ffi.Pointer<ffi.Char> _stringToFFIPointer(String s) {
  return s.toNativeUtf8().cast<ffi.Char>();
}

ffi.Pointer<ffi.Char> _hashMapToFFIPointer(Map<String, String> dict) {
  String val = dict.entries.map((e) => "${e.key}:${e.value}").join(';');
  return _stringToFFIPointer(val);
}

/// Returns the Dart equivalent of an empty `char**`
ffi.Pointer<ffi.Pointer<ffi.Char>> _getEmptyStringPointer() {
  return calloc<ffi.Pointer<ffi.Char>>();
}

/* Region: Utility Functions  */

/// Loads the dynamic library using an appropriate extension for the given platform
String getLibraryPath() {
  var libraryPath =
      path.join(Directory.current.path, 'libs', 'liboauthtool.so');
  if (Platform.isMacOS || Platform.isIOS) {
    libraryPath =
        path.join(Directory.current.path, 'libs', 'liboauthtool.dylib');
  } else if (Platform.isWindows) {
    libraryPath = path.join(Directory.current.path, 'libs', 'liboauthtool.dll');
  }
  return libraryPath;
}

/// Loads the dynamic library from the given library path
ffi.DynamicLibrary loadLibrary(String libraryPath) {
  final dylib = ffi.DynamicLibrary.open(libraryPath);
  return dylib;
}

