package solana

import (
	"context"

	"github.com/blocto/solana-go-sdk/client"
	"github.com/blocto/solana-go-sdk/common"
	"github.com/blocto/solana-go-sdk/rpc"
)

const (
	Devnet  Network = "devnet"
	Testnet Network = "testnet"
	Mainnet Network = "mainnet"
)

type Network string

func (n Network) RpcUrl() string {
	switch n {
	case Devnet:
		return rpc.DevnetRPCEndpoint
	case Testnet:
		return rpc.TestnetRPCEndpoint
	case Mainnet:
		return rpc.MainnetRPCEndpoint
	default:
		return string(n)
	}
}

func LoadClient(network Network) *client.Client {
	return client.NewClient(network.RpcUrl())
}

type W3Site struct {
	Contract common.PublicKey
	Client   *client.Client
}

func (s *W3Site) UrlAccount(path string) (common.PublicKey, error) {
	seeds := [][]byte{
		[]byte("w3-site-url"),
		[]byte(path),
	}
	return common.CreateProgramAddress(seeds, s.Contract)
}

func (s *W3Site) LoadAccountContent(account *common.PublicKey) ([]byte, error) {
	info, err := s.Client.GetAccountInfo(context.Background(), account.ToBase58())
	if err != nil {
		return nil, err
	}
	return info.Data, nil
}
