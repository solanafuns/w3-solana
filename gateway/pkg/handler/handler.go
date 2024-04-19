package handler

import (
	"bytes"
	"encoding/json"
	"fmt"
	"mime"
	"net/http"
	"strings"

	"github.com/sirupsen/logrus"
	"github.com/solanafuns/w3-solana/gateway/pkg/solana"
)

func Handle(w http.ResponseWriter, r *http.Request) {
	host := r.Host
	logrus.Info("host: ", host)
	segments := strings.Split(host, ".w3sol.xyz")
	if len(segments) >= 2 {
		nameHost := segments[0]
		logrus.Infof("nameHost is %s", nameHost)
		client := solana.GetClient()
		if config, err := client.GetSiteConfig(nameHost); err != nil {
			w.WriteHeader(http.StatusBadRequest)
			w.Write([]byte(fmt.Sprintf("site config not found!!! \n %s ", err.Error())))
			return
		} else {
			client.Program = config.Program

			if r.RequestURI == "/_config" {
				configBytes, _ := json.Marshal(config)
				w.Write(configBytes)
				return
			}
		}
		logrus.Info("host: ", host, " name: ", nameHost, " program: ", client.Program.ToBase58())

		if account, err := client.UrlAccount(r.RequestURI); err != nil {
			w.WriteHeader(http.StatusNoContent)
			w.Write([]byte(err.Error()))
		} else {
			if content, err := client.LoadAccountContent(&account); err != nil {
				w.WriteHeader(http.StatusInternalServerError)
				w.Write([]byte(err.Error()))
			} else {
				if pageContent, err := solana.ParsePageContent(content); err == nil {
					jsonMeta, _ := json.Marshal(pageContent)
					logrus.Infof("parse content done %s ", string(jsonMeta))
					if len(pageContent.RawData) > 0 {
						logrus.Info("has RawData")
						w.Header().Set("W3-Solana-Resolver", client.NameResolver.ToBase58())
						w.Header().Set("W3-Solana-Program", client.Program.ToBase58())
						w.Header().Set("W3-Solana-Account", account.ToBase58())
						w.Header().Set("W3-Solana-Network", client.Network())
						segments := strings.Split(r.RequestURI, ".")
						if len(segments) > 1 {
							fileExtension := segments[len(segments)-1]
							w.Header().Set("Content-Type", mime.TypeByExtension("."+fileExtension))
						}
						w.WriteHeader(http.StatusOK)
						w.Write(pageContent.RawData)
					} else if pageContent.TrunkPage.Trunks > 0 {
						pageBuffer := bytes.NewBuffer(nil)
						for i := 0; i <= int(pageContent.TrunkPage.Trunks); i++ {
							if trunkAccount, err := client.UrlTrunkAccount(r.RequestURI, uint8(i)); err == nil {
								if trunkContent, err := client.LoadAccountContent(&trunkAccount); err == nil {
									pageBuffer.Write(trunkContent)
								}
							}
						}
						w.WriteHeader(http.StatusOK)
						w.Write(pageBuffer.Bytes())
					}
				} else {
					w.WriteHeader(http.StatusInternalServerError)
					w.Write([]byte(err.Error()))
				}
			}
		}
	} else {
		w.WriteHeader(http.StatusNotFound)
		w.Write([]byte("Not Found"))
	}

}
