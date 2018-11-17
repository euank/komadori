import { connect } from 'react-redux';
import { doListOtherUser } from '../actions';
import EditUserComponent from '../components/EditUser';

const mapStateToProps = (state, ownProps) => {
  const { uuid } = ownProps.match.params;
  console.log(state.users, uuid);
  return {
    otherUser: state.users[uuid],
    uuid,
  };
};

const mapDispatchToProps = dispatch => ({
  getUser: (uuid) => {
    dispatch(doListOtherUser(uuid));
  },
});

const EditUser = connect(
  mapStateToProps,
  mapDispatchToProps,
)(EditUserComponent);

export default EditUser;
