package solana

import (
	"github.com/blocto/solana-go-sdk/common"
	"github.com/near/borsh-go"
)

type NameConfig struct {
	Name      string
	Program   common.PublicKey
	Creator   common.PublicKey
	CreatedAt uint64
}

func ParseConfig(data []byte) (*NameConfig, error) {
	b := new(NameConfig)
	if err := borsh.Deserialize(b, data); err != nil {
		return nil, err
	}
	return b, nil
}
