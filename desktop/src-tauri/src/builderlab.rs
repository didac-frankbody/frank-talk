use std::{collections::HashMap, sync::Mutex, time::Duration};

use axum::{
    extract::{Path, Query, State as AxumState},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tauri_plugin_opener::OpenerExt;
use tokio::{net::TcpListener, sync::oneshot};
use url::Url;

const BUILDERLAB_API_BASE_URL: &str = "https://app.builderlab.xyz/api/goose";
const LOGIN_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const BB_SESSION_CREDENTIAL_HEADER: &str = "X-BB-Session-Credential";
// Builderlab enforces an Origin check on the identity bind endpoints. Browsers
// attach this automatically; the desktop reqwest client must set it explicitly
// or challenge/verify fail with `invalid_origin`. It also seeds the challenge
// body's `origin` field so both agree.
const BUILDERLAB_ORIGIN: &str = "https://app.builderlab.xyz";
// The page the user's browser lands on after Builderlab sign-in. It is served
// standalone, with no access to the app's asset server, so the mark is inlined.
const AUTH_COMPLETE_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>frank talk authentication complete</title>
  <style>
    :root {
      color-scheme: light;
      font-family: ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      color: #3f2a2d;
      background: #3f2a2d;
    }

    * {
      box-sizing: border-box;
    }

    body {
      min-height: 100vh;
      min-height: 100dvh;
      margin: 0;
      display: grid;
      place-items: center;
      padding: 24px;
      background-color: #3f2a2d;
      background-image: radial-gradient(circle, rgba(255, 251, 250, 0.12) 1.2px, transparent 1.3px);
      background-size: 37px 37px;
    }

    main {
      width: min(100%, 560px);
      padding: clamp(32px, 8vw, 64px);
      border: 2px solid #3f2a2d;
      border-radius: 28px;
      background: #fffbfa;
      box-shadow: 8px 8px 0 rgba(63, 42, 45, 0.55);
    }

    .mark {
      display: block;
      width: 72px;
      height: 72px;
      margin-bottom: 40px;
      border-radius: 16px;
    }

    .eyebrow {
      display: inline-flex;
      align-items: center;
      min-height: 32px;
      margin: 0 0 20px;
      padding: 6px 14px;
      border-radius: 999px;
      background: #ffb6a5;
      font-size: 14px;
      font-weight: 600;
      letter-spacing: 0.01em;
    }

    h1 {
      max-width: 440px;
      margin: 0;
      font-size: clamp(40px, 9vw, 64px);
      font-weight: 600;
      letter-spacing: -0.055em;
      line-height: 0.95;
    }

    p {
      max-width: 390px;
      margin: 24px 0 0;
      font-size: 18px;
      letter-spacing: -0.02em;
      line-height: 1.45;
      color: rgba(63, 42, 45, 0.68);
    }

    @media (max-width: 480px) {
      body {
        padding: 16px;
      }

      main {
        padding: 32px 28px 36px;
        border-radius: 22px;
        box-shadow: 6px 6px 0 rgba(63, 42, 45, 0.55);
      }

      .mark {
        width: 60px;
        height: 60px;
        margin-bottom: 32px;
      }
    }
  </style>
</head>
<body>
  <main>
    <img class="mark" alt="frank talk" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABwCAYAAADG4PRLAAAQAElEQVR4AeydCZiP1R7HfzNjGYNslUI1lhIppGhR0XK7D7kt96mr7SZpUUKUppQet7QpDaVoedJtLz3cVpG1kqUnhGxZHiGyZDAIM+73c5r3339WjOH//jnzvL856/u+53y//985v3PO+5430fbvL0Gnl65Ro0ZKzZo1q5188smtGjRo0LlBgwbp8n8sd4ZkqWStJFOS3bBhw92HslBH1X2rXOpM3WfJ/6niBsm988QTTzy/bt26R9cQZsKujAQM5RTvKBaB9erVK3vSSSfVlHuWCpV2xBFHjJDMT0hIGCd5UdItMTGxndwmklTJkZIUyX4VtnhVPLhnUUfVvZxc6kzdT5O/reLulju4VKlSE8qUKTNfeI0QqWnCsaWkZmpqanJxSrpPBPKrEWlNk5KSOqpAQ1SQz1Wo/8j/N7kU2BEkv6WkpFi1atXsmGOOMWmnHX/88aZCWu3atQ9poY7UlTpTdzAACzCBILkcVcBM0ldYfiYZmpyc3ElENgZj8u2t7C2BpaT6TSpXrtyrdOnSr+mGg3Tzy3STyhJHVt26da1Vq1Z27bXXWufOne2ee+6xXr162QMPPGAPPfSQPfzww/bII49Ynz59DmmhjtS1d+/eru5g0KNHD7vzzjvt+uuvtwsvvNCkBA4zsBObFSVtJenC9VVhfD9YK620ZI/HHgnULyq5fv36baT6z+zevbuHbtRUVy0l1yCtffv2jhxIuu+++6xr166OwOuuu84uu+wyu/jii+3888+3c845x5o3b35YCHW94IILXN3BgB/1HXfc4bC599573Q8avMBIWmcizoRnkuQMYdsTrIX55XujjUUSWKtWrXJly5btqRs8pwu30g0qyrVTTjnFadHTTz9td911l7Vt29YRI7JNbbtJO8nmJQoBMKlQoYKdcMIJduaZZ7ofNy3VU089ZY8++qiddtppQe7y8rRS/v6VKlVKq1OnTiWFCz2KIjCpYsWKNJVpOruOyEuiPe/evbulp6fbFVdcYeqErWrVqu4XpDz+2AcEpBQOOzSwTZs2NnDgQOvbt69Vr1490EYMoB5SoEG6bJKkwCOxoFg0T9blEJHWSVKBmzVr1szd5JZbbjGptsmAKehUH1cMBGRX2NFHH21XXXWVvfDCCwbWYK5LlRf+/xYXbwrzFIXzHfkIVAdbVppHX9ee3PoF2CWXXGKPP/64NW3a1DePgHKARM2maYxs/fr1c01suXLlgjv9U11TbxQriAjcvASq/yzFkOBmZXCah2XZXc2mTlZUcHj3QCJw3HHHOdvi0ksvdS2dtLCMpEP58uWx/EtF3zsXgTJfGyljN1mbqWRq0qSJGw5wQcJeDh4CjCMxELHcc+5aXRraHY5yws6JECgNKyf1a6fY5iIx6aijjvLkCYxYHur3nCbCBZxIGqu/vELNbIWgXBEC1e/VVWQbZXJDhVtvvdUaN26sKH/EEoFGjRoZhmNOGcqrdfxHdnZ2ZMzhCMRwUWRLkdeMjIzzLrroIm+wAEaMRc2mM2iCcaI4OlXSUtrprFJHoEzWasr4d5W1tBLddBhjPoX9EQIEZIEa03Bwo+JgxFwkC7WG/AaBCVlZWSco0FpiGvmbxh2mtpaglxAgIAVzMzUyYILStBA/dHmJECjbpVRrsXsEqWeccYbRaeL3Eh4EKleubOeee64rkLhieq2VZm3KJaotLa3YCyRuhhzDhcyEvYQHAeZRTz/9dMdRTqlayPBMSUxJSWG65nQijz32WENQWcIlLv6CxUZANoprGeGHi0gLG4unCopPrK+IahJjwM6cHH4v4UOAhQM4yilZFZHYKJF/EreSrubUraLnZPBOyBCga4vSQDg7CQ08mXKKRCOD5tsIegkhAho6WJUqVVhucqUTZ/USNbKvQyhIFKMEvYQQAbiBQAwaiifuUhlGHE8gOTnZZNXg9RJiBBjUo2wUURpYkz6QMYVbtggSSPQSTgRQNNZoc0pXCQ3kGQz3WIRG9znx3gkrAjwJoZmXoHgVPIEBFHHiQp7Gf0Fpy0GgW7engyQxSAmT68vyFwJwFEVgMn0g44mIafpXVu8LOwIyYhLQwLCX05evCAQ8gUWAEw9JnsB4YKmIMnoCiwAnHpI8gfHAUhFl9AQWAU48JHkC44GlIsp4KBNYRLUPnSRPYJxz6Qn0BIYbAS162tKlS927jY899phFy5NPPmljx44NdwX2ULpDXgN37dpl48aNszfffNM+/PDDXPLee++5+M2bN+8BpvAmh5LA7OxspzVfffWVLVq0yCChuBCigdu3b7dt27a563CtQHbu3Gnr1693acW9fqzPCyWBP//8sz344IPWrVs3907A8uXLi40Ti9S8unz33Xe7t3x40+fUU091C9hcFIJx41VCSSBN2m+//eYwzczMtC1btjh/cf5pycU9rMw2H+zXgvCqeNSaWnEuG5pzQklgvGvFwWQ3lAQeTADy3yu+YjyB8cVXvtJ6AvNB8mfEihUrbOLEiTZhwoQC5euvv7YlS5ZYrJt7T+CffOX6Dyndu3d3ljCb1hUkaWlp9uyzz9qaNWtynXuwAzElEKDmzp3rBtOvvPKKBfLJJ5/ksjxHjBgRSQvy5HU/+ugj27hxY4ngR7nQLq5XlDCG3B8LuSQKG1MCGS68+uqrxpRWenq6pefI8OHDcxH4wQcfRNKCPHndAQMG2OjRo0sCE+MRyyuvvNLtaxq8hxBcmAdra9eubfXr1zdeuKxevXqQFBM3pgRmZWW5d/FLYkzG85JISaHIPqdsncmeAcE1eS+B7SNfe+01GzZsmPXs2TPm75PElMBKlSq5Dd5uu+0269ixY0TYvjL6Nbd27dpF0qLzBX5mVzp16mQtWrQIsN4vl6m2L774woYMGWI//vijuxbv5XXo0MHY7xOtg8yS+OG5i+/Hv5gSSFMF6Lfffrsx1RUIU1/Rb0pdffXVudKDfIHbpUsXY/NUtqfaDyzcqcybvv/++271YsGCBS6OJpONj9i4FeJcZEj+xZRAMGCqi/lK+pZA8jaFedODfNFuSWhD0CdjIP36668Uz1JTU92Ph6aTHxXldQl5/8UoHHMCY1TvfLfFomSvzrfeesvWrl3r0tE8hhCtW7c2mvSwkUchD3sCGTKsWrXK2D763XffNbQQYNB6+l6aeDSduDDKYU/g77//bs8//7x99tlnhlUcaBlrhfSFEzQTgz+M5FGmw57AjIwMmzJlipsSw0BhQ3I2ewAclrSwROfMmePIJS5sctgTGBDCFis33XST2+6YIQnNJs0rlujQoUMNlycFgvxhcUNJIMOLaKuScdmBBAzyII1vYDDeY9jCkIFy0KxOnTrVGLhjmULqgSzLvl47lATyEn9KitsO09UnsApdoIT/sa0mX1VhmEDTSR/I9NnNN99sfA6A2+3YsSPyYFSs5z4pT7SEkkCar2gC58+fH13mfH60Yt26dcakN/0V4XyZColgbIelyTAhOsuRRx5pzBCRRjwPRfFUG/O0hMMioSSQKTaasgCkkSNH2oYNG5whATkI/RGagdnP44E0e3zO5plnnnH9VXAuLvkRmkMEP/EIGofkjSeNeVBmeRgPEmaWhiWkMWPGuCfcCjqHfPsi+5s3lARWrVrVfRWGPVGoINYgQPKY4bx58wyN/Oabb2zw4MF2zTXXuId1yUPeX375xQI/YYRhAMtWo0aNcsMFHvTlB0AaTSLPjTJcYDwIKcRDMk/DrV692n2sinFhEM8Hrlgh4aFg7hecQ/rBllASyFTaWWedZWz4jSEBKLNmzTKeKLvxxhvthhtucF8DYykKkElHi+jDzj77bKtVqxZRTiBi5cqVdv/990e+pgb5gWFE/8pHTXiE8e23344M5P/44w9Dm/mgFxrHj8BdUP8YevBxDsrDygRrhoqOyRFKAkGiYcOGhiHBmhuEEofQjCEQQxhhdYCPZPDJOya4gyaPNASy0DT8hQnX27p1q2umyUN406ZNeAsV8kC018ACIGIY0bJlS7d8Q/PJJ+zq1avnNj1lS2g+GsW3+LAgadIgj0VYyEQbg0viJ458jPMKE1YbsDoxajgXQ4q8HbSEhFuQsIzF0hd9NufEQkKrgYCB5vEUNTu286Q285VMeyF8ti0tLc0AGHJpNiGd8/IKpFx++eXGslVhAhk88AtxnM+1zjvvPGeJFnYOY0eaeoY9nBMLCTWBASAMKdiplscYIBRBA1n/w/xHy4K8BbmkYxChKYUJJAeGCtfgHMKF5SeeqTfykD9WEhcExgqceLivJzAeWCqijJ7AIsAJcVKkaJ7ACBTx6fEExidvkVJ7AiNQxKfHExifvEVK7QmMQBGfHk9gfPIWKbUnMAJFfHo8gfHJW6TUnsAIFPHp2TcC47OOh3SpPYFxTi9fL9tNHVhdxvUSPwiIs90QuJ0i85APjx7g9xJeBOAo6hGO7Xy5ZSvF9QSCQviFh6uiCNxKH+j2WoRZnrMMfxUO7xJCIFzloJBJE5pBgASe9sLvJbwIwFGUom2mCV1BcUngKWf8XsKLABzx+CMllBGzEg1cQoBIXnakLyTsJXwIwA0csQUnpUtMTFyGBro3R8SmRSeSoaTEX6dkEKCV3Lhxo0EkV5S7CCNmochzY0HeA+AlEhK9hA8ByOMdRUoGZ7JG59GEzlDERonxnkHeF0OI9xIOBFAuOKI0CQkJGdLABYmbNm3aJjanE8nbOTAsZgl6CRECIsvtjAhHFEvhGZCYqIhdivhOYhgyvCApUgl6CRECGC68oQVHFEtKNy0jI2MbfeBOsTlWEW48OH369MhGN2T0Eg4E6P+++87pGQXaIr4mrV271s3E7BaBSxU7WWKLFy+22bNnW9RgkWgvMUSALo1dMhCKIfKmJCUlLZI/Gw3kdeF1ivxckk1m3gOHcWXwRwgQ4P38N954w727KI6yJONk0KyiaI7AZcuWsSKBfs4lki0W2W5RmknQSwwRgIOJEyfaDz/8EJQC42X8mjVrMolwBOLRIJ4B/cfyu4TXX3/dvYuucKwOf18hwKeHnnvuOfmM3aS2yfL8fN26dXNchP5FCIRRzXIPl3pOkWSpgzS2Qmb3duXzRwwQ4N1+dsVgaAcnKsIMLTqMEDeRT9lECFQiH5qaK5UdKr+b4J45c6a99NJLxoUU54+DiACzYmzi8P333wd3XStuXpRGum4uiMxFoCJ3ynj5XBn/K38mBs2XX37ptvPwJAqRg3RobG4vv/yy2xKFTRR02x1qOt8SNyPl3ymJHHkJZLSfKTV9Vir7EbmwgDBo+vbt6/aPFrlEezkACIAtQwWwHjlypDF4z7nNxxrA96ObywlHnHwEkrJkyZKMefPmdRSJ7yqsIeEO+/bbb93n4NjOSjMAzqRVmj9KAAFaOtb5GL517tzZ2Mcm0DxdfvhPP/3UXiMFN1+tcK6jQAJzcmSJqE4isb9klSSLie4+ffoYn3JjlyImVpl245eTc4539hIBMIM0MIgHzQAAAgdJREFUdnriexdsg4LmScuwNrN0mdXKM0iad6v8hOXkP4oi0NQWb1UT+pjI66pTp0rcEINxIrsUsbsRe2mOHz/ezd7QT6L2urGy+iMaATABG2FqzDeztRdGCjiyi1QwzhPW23TeVI0Ieq1fv75vYZqnPO4okkBy6ALb58+f/z9d8C5d/CnJFMXvlGsLFy60YcOGWVpamrE5OFtWpaenO8v1nXfeMT6hw/5mDEQnT55s06ZNOyyEuk6aNMmo+6effmrsxc3Ov2DzxBNPOKwgDQLVPLruSHhmS2YL2wFgLWzfix4uKL7AY48E5py1S+brTE3fDFB73VXSW/GjdMPNct0qBnOoEyZMMIhj6DFw4EAbMGCA22+sf//+xsY8jCsPvDxpsb4HdWVTIvZaAwOwABOwYWM9PjGrphHoaC43STvHCcuHFNFFBuTTYC1/LmtT4QKPvSXQnaz2OVO/jOlquwfLsumim14t6acCjJe7QbKbjHLdt4/oM2nj2R2QQmNhHQ5CXakzdRdmbgM9YQQ0EAZGGxUGs/5ShmuF5R2aCRskw3GSzt3kMu7lv30iMLim2vGtutFiNa1jpJVPSOX/pV9OcxWmrQrWUwQOkYuGzpF/uYTJchaOKXxwmUPSVV05qCs/aOo+R1iMVmVflnuf3KuEVTMw05TYf6QQo2T1LxLRzr5Q+j4d/wcAAP//WbC83wAAAAZJREFUAwCzI65raUpbFAAAAABJRU5ErkJggg==">
    <div class="eyebrow">Authentication complete</div>
    <h1>You&rsquo;re signed in.</h1>
    <p>You can close this window and head back to frank talk.</p>
  </main>
</body>
</html>"#;

#[derive(Default)]
pub(crate) struct BuilderlabSession(Mutex<Option<StoredSession>>);

#[derive(Default)]
pub(crate) struct BuilderlabLogin(Mutex<Option<PendingLogin>>);

struct PendingLogin {
    id: uuid::Uuid,
    cancel: oneshot::Sender<()>,
}

struct StoredSession {
    credential: String,
}

#[derive(Debug, Deserialize)]
struct LoginExchangeResponse {
    session_credential: String,
    expires_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BuilderlabAuthInfo {
    expires_at: String,
    email: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AuthMeResponse {
    email: Option<String>,
    name: Option<String>,
    expires_at: String,
}

struct CallbackState {
    nonce: String,
    sender: Mutex<Option<oneshot::Sender<Result<String, String>>>>,
}

async fn login_callback(
    Path(nonce): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    AxumState(state): AxumState<std::sync::Arc<CallbackState>>,
) -> Response {
    if nonce != state.nonce {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    }

    let result = match query.get("code").filter(|code| !code.is_empty()) {
        Some(code) => Ok(code.clone()),
        None => Err(query
            .get("error_description")
            .or_else(|| query.get("error"))
            .cloned()
            .unwrap_or_else(|| "Authentication callback did not include a code".to_owned())),
    };
    if let Some(sender) = state
        .sender
        .lock()
        .expect("callback sender poisoned")
        .take()
    {
        let _ = sender.send(result);
    }

    Html(AUTH_COMPLETE_HTML).into_response()
}

fn api_url(path: &str) -> Result<Url, String> {
    Url::parse(&format!("{BUILDERLAB_API_BASE_URL}{path}"))
        .map_err(|error| format!("invalid Builderlab API URL: {error}"))
}

fn login_url(return_to: &str) -> Result<Url, String> {
    let mut login_url = api_url("/v1/auth/login")?;
    login_url
        .query_pairs_mut()
        .append_pair("type", "cli")
        .append_pair("product", "buzz")
        .append_pair("returnTo", return_to);
    Ok(login_url)
}

async fn authenticated_user(
    client: &reqwest::Client,
    credential: &str,
) -> Result<AuthMeResponse, String> {
    let response = client
        .get(api_url("/v1/auth/me")?)
        .header(BB_SESSION_CREDENTIAL_HEADER, credential)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|error| format!("Builderlab session check failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "Builderlab session check failed with HTTP {}",
            response.status()
        ));
    }
    response
        .json()
        .await
        .map_err(|error| format!("invalid Builderlab session response: {error}"))
}

#[tauri::command]
pub(crate) async fn start_builderlab_login(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
    login: tauri::State<'_, BuilderlabLogin>,
) -> Result<BuilderlabAuthInfo, String> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|error| format!("could not start local authentication callback: {error}"))?;
    let port = listener
        .local_addr()
        .map_err(|error| format!("could not read local authentication callback: {error}"))?
        .port();
    let nonce = uuid::Uuid::new_v4().simple().to_string();
    let return_to = format!("http://127.0.0.1:{port}/callback/{nonce}");
    let (sender, receiver) = oneshot::channel();
    let callback_state = std::sync::Arc::new(CallbackState {
        nonce: nonce.clone(),
        sender: Mutex::new(Some(sender)),
    });
    let router = Router::new()
        .route("/callback/{nonce}", get(login_callback))
        .with_state(callback_state);
    let server = tokio::spawn(async move {
        let _ = axum::serve(listener, router).await;
    });

    let login_url = login_url(&return_to)?;
    if let Err(error) = app.opener().open_url(login_url.as_str(), None::<&str>) {
        server.abort();
        return Err(format!("could not open Builderlab authentication: {error}"));
    }

    let login_id = uuid::Uuid::new_v4();
    let (cancel_sender, mut cancel_receiver) = oneshot::channel();
    {
        let mut pending = login.0.lock().map_err(|error| error.to_string())?;
        if let Some(previous) = pending.take() {
            let _ = previous.cancel.send(());
        }
        *pending = Some(PendingLogin {
            id: login_id,
            cancel: cancel_sender,
        });
    }

    let exchange_code = tokio::select! {
        result = tokio::time::timeout(LOGIN_TIMEOUT, receiver) => match result {
            Ok(Ok(Ok(code))) => code,
            Ok(Ok(Err(error))) => {
                server.abort();
                return Err(error);
            }
            Ok(Err(_)) => {
                server.abort();
                return Err("local authentication callback stopped unexpectedly".to_owned());
            }
            Err(_) => {
                server.abort();
                return Err("Builderlab authentication timed out".to_owned());
            }
        },
        _ = &mut cancel_receiver => {
            server.abort();
            return Err("Builderlab authentication canceled".to_owned());
        }
    };
    server.abort();

    let response = app_state
        .http_client
        .post(api_url("/v1/auth/login/exchange")?)
        .json(&serde_json::json!({ "code": exchange_code }))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|error| format!("Builderlab code exchange failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "Builderlab code exchange failed with HTTP {}",
            response.status()
        ));
    }
    let exchanged: LoginExchangeResponse = response
        .json()
        .await
        .map_err(|error| format!("invalid Builderlab code exchange response: {error}"))?;
    if exchanged.session_credential.is_empty() {
        return Err("Builderlab code exchange returned an empty credential".to_owned());
    }

    let me = authenticated_user(&app_state.http_client, &exchanged.session_credential).await?;
    if exchanged.expires_at != me.expires_at {
        return Err("Builderlab session expiry did not match code exchange".to_owned());
    }
    let info = BuilderlabAuthInfo {
        expires_at: me.expires_at.clone(),
        email: me.email,
        name: me.name,
    };
    {
        let mut pending = login.0.lock().map_err(|error| error.to_string())?;
        if pending
            .as_ref()
            .is_none_or(|pending| pending.id != login_id)
        {
            return Err("Builderlab authentication canceled".to_owned());
        }
        *pending = None;
    }
    *session.0.lock().map_err(|error| error.to_string())? = Some(StoredSession {
        credential: exchanged.session_credential,
    });
    Ok(info)
}

#[tauri::command]
pub(crate) async fn get_builderlab_auth(
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<Option<BuilderlabAuthInfo>, String> {
    let stored = session
        .0
        .lock()
        .map_err(|error| error.to_string())?
        .as_ref()
        .map(|stored| stored.credential.clone());
    let Some(credential) = stored else {
        return Ok(None);
    };
    match authenticated_user(&app_state.http_client, &credential).await {
        Ok(me) => Ok(Some(BuilderlabAuthInfo {
            expires_at: me.expires_at,
            email: me.email,
            name: me.name,
        })),
        Err(error) => {
            *session
                .0
                .lock()
                .map_err(|lock_error| lock_error.to_string())? = None;
            Err(error)
        }
    }
}

#[tauri::command]
pub(crate) fn cancel_builderlab_login(
    login: tauri::State<'_, BuilderlabLogin>,
) -> Result<(), String> {
    if let Some(pending) = login.0.lock().map_err(|error| error.to_string())?.take() {
        let _ = pending.cancel.send(());
    }
    Ok(())
}

#[tauri::command]
pub(crate) fn clear_builderlab_auth(
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<(), String> {
    *session.0.lock().map_err(|error| error.to_string())? = None;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct NostrIdentityChallenge {
    challenge_id: String,
    nonce: String,
    verification_code: String,
    origin: String,
    expires_at: String,
}

async fn authenticated_json(
    client: &reqwest::Client,
    session: &BuilderlabSession,
    method: reqwest::Method,
    path: &str,
    body: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let credential = session
        .0
        .lock()
        .map_err(|error| error.to_string())?
        .as_ref()
        .map(|stored| stored.credential.clone())
        .ok_or_else(|| "Sign in to Builderlab first".to_owned())?;
    let response = client
        .request(method, api_url(path)?)
        .header(BB_SESSION_CREDENTIAL_HEADER, credential)
        .header(reqwest::header::ORIGIN, BUILDERLAB_ORIGIN)
        .json(&body)
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .map_err(|error| format!("Builderlab request failed: {error}"))?;
    let status = response.status();
    let value: serde_json::Value = response
        .json()
        .await
        .map_err(|error| format!("invalid Builderlab response: {error}"))?;
    if !status.is_success() {
        // Builderlab error responses carry a structured `{ error: { code,
        // message, setup_needed, ... } }` body. Pass those through as `Ok` so the
        // frontend's typed handling and friendly per-code messages apply, instead
        // of surfacing a raw JSON blob. Only fall back to a plain string when the
        // body isn't the expected shape.
        if value.get("error").is_some() {
            return Ok(value);
        }
        return Err(format!("Builderlab request failed (HTTP {status})."));
    }
    Ok(value)
}

#[tauri::command]
pub(crate) async fn get_builderlab_nostr_identity(
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/nostr-identities/current",
        serde_json::json!({}),
    )
    .await
}

#[tauri::command]
pub(crate) async fn bind_builderlab_nostr_identity(
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    let challenge_value = authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/nostr-identities/challenge",
        serde_json::json!({ "origin": BUILDERLAB_ORIGIN }),
    )
    .await?;
    // A structured error here (e.g. missing_mapping) arrives as an object with an
    // `error` field rather than a challenge — hand it straight back so the
    // frontend maps it to a friendly message instead of hitting a deserialize
    // failure below.
    if challenge_value.get("error").is_some() {
        return Ok(challenge_value);
    }
    let challenge: NostrIdentityChallenge = serde_json::from_value(challenge_value)
        .map_err(|error| format!("invalid Nostr identity challenge: {error}"))?;
    let keys = app_state.signing_keys()?;
    let event = crate::commands::build_nostr_identity_binding_event(
        &keys,
        &challenge.challenge_id,
        &challenge.nonce,
        &challenge.verification_code,
        &challenge.origin,
        &challenge.expires_at,
    )?;
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/nostr-identities/verify",
        serde_json::json!({
            "challenge_id": challenge.challenge_id,
            "nonce": challenge.nonce,
            "signed_payload": nostr::JsonUtil::as_json(&event),
        }),
    )
    .await
}

#[tauri::command]
pub(crate) async fn delete_builderlab_nostr_identity(
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/nostr-identities/delete",
        serde_json::json!({}),
    )
    .await
}

#[tauri::command]
pub(crate) async fn list_builderlab_communities(
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/communities/list",
        serde_json::json!({}),
    )
    .await
}

#[tauri::command]
pub(crate) async fn check_builderlab_community_name(
    name: String,
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/communities/availability",
        serde_json::json!({ "name": name }),
    )
    .await
}

#[tauri::command]
pub(crate) async fn create_builderlab_community(
    name: String,
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/communities",
        serde_json::json!({ "name": name }),
    )
    .await
}

#[tauri::command]
pub(crate) async fn archive_builderlab_community(
    community_id: String,
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/communities/archive",
        serde_json::json!({ "community_id": community_id }),
    )
    .await
}

#[tauri::command]
pub(crate) async fn unarchive_builderlab_community(
    community_id: String,
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/communities/unarchive",
        serde_json::json!({ "community_id": community_id }),
    )
    .await
}

#[tauri::command]
pub(crate) async fn transfer_builderlab_community(
    community_id: String,
    transferee_npub: String,
    app_state: tauri::State<'_, crate::app_state::AppState>,
    session: tauri::State<'_, BuilderlabSession>,
) -> Result<serde_json::Value, String> {
    // The Builderlab transfer endpoint expects camelCase keys, unlike the
    // archive/unarchive endpoints which take `community_id`; mirror the web
    // client's payload exactly.
    authenticated_json(
        &app_state.http_client,
        &session,
        reqwest::Method::POST,
        "/v1/buzz/communities/transfer",
        serde_json::json!({
            "communityId": community_id,
            "transfereeNpub": transferee_npub,
        }),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_complete_page_uses_frank_talk_brand() {
        for expected in [
            "<title>frank talk authentication complete</title>",
            // ink, Original Pink, off-white — the page's field, eyebrow and card.
            "#3f2a2d",
            "#ffb6a5",
            "#fffbfa",
            "alt=\"frank talk\"",
            "head back to frank talk",
        ] {
            assert!(
                AUTH_COMPLETE_HTML.contains(expected),
                "authentication complete page is missing {expected}"
            );
        }
    }

    #[test]
    fn api_paths_stay_on_builderlab_api_origin() {
        let login = api_url("/v1/auth/login").unwrap();
        assert_eq!(
            login.origin().ascii_serialization(),
            "https://app.builderlab.xyz"
        );
        assert_eq!(login.path(), "/api/goose/v1/auth/login");
    }

    #[test]
    fn login_defaults_to_auth0_login() {
        let login = login_url("http://127.0.0.1:1234/callback/nonce").unwrap();
        let query: HashMap<_, _> = login.query_pairs().into_owned().collect();

        assert_eq!(query.get("type").map(String::as_str), Some("cli"));
        assert_eq!(query.get("product").map(String::as_str), Some("buzz"));
        assert_eq!(
            query.get("returnTo").map(String::as_str),
            Some("http://127.0.0.1:1234/callback/nonce")
        );
        assert!(!query.contains_key("screen_hint"));
    }
}
